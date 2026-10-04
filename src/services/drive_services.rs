//! Opérations "explorateur de fichiers" : lister, créer/renommer/supprimer.
//! Tous les chemins reçus du client sont relatifs à `uploads/` et validés ici.

use crate::models::drive::DriveEntry;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use std::path::{Path, PathBuf};

/// Dossier racine du drive (le même que celui servi sur `/uploads`).
#[cfg(feature = "server")]
pub fn uploads_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("uploads")
}

/// Transforme un chemin relatif "a/b/c" en chemin disque, en refusant tout ce qui
/// pourrait sortir de `uploads/` (`..`, chemins absolus, `C:` sous Windows, etc.).
#[cfg(feature = "server")]
pub fn resolve_path(rel: &str) -> Result<PathBuf, String> {
    let mut path = uploads_root();
    for part in rel.split('/').filter(|s| !s.is_empty()) {
        if part == "." || part == ".." || part.contains(['\\', ':', '\0']) {
            return Err("Chemin invalide".into());
        }
        path.push(part);
    }
    Ok(path)
}

/// Valide un nom saisi par l'utilisateur (dossier ou renommage).
#[cfg(feature = "server")]
pub fn clean_name(name: &str) -> Result<String, String> {
    let name = name.trim();
    if name.is_empty() || name == "." || name == ".." {
        return Err("Nom invalide".into());
    }
    if name.len() > 255 || name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0']) {
        return Err("Le nom contient des caractères interdits ( / \\ : * ? \" < > | )".into());
    }
    Ok(name.to_string())
}

/// Nettoie un nom de fichier uploadé (on remplace au lieu de refuser).
#[cfg(feature = "server")]
pub fn sanitize_upload_name(raw: &str) -> String {
    let base = raw.rsplit(['/', '\\']).next().unwrap_or("");
    let cleaned: String = base
        .chars()
        .map(|c| if matches!(c, ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0') { '_' } else { c })
        .collect();
    let cleaned = cleaned.trim().to_string();
    if cleaned.is_empty() || cleaned == "." || cleaned == ".." {
        "fichier".to_string()
    } else {
        cleaned
    }
}

/// Renvoie un chemin libre dans `dir` : "photo.jpg", puis "photo (1).jpg", etc.
#[cfg(feature = "server")]
pub async fn unique_path(dir: &Path, name: &str) -> PathBuf {
    let first = dir.join(name);
    if !tokio::fs::try_exists(&first).await.unwrap_or(false) {
        return first;
    }
    let (stem, ext) = match name.rfind('.') {
        Some(i) if i > 0 => (&name[..i], &name[i..]),
        _ => (name, ""),
    };
    let mut n = 1;
    loop {
        let candidate = dir.join(format!("{stem} ({n}){ext}"));
        if !tokio::fs::try_exists(&candidate).await.unwrap_or(false) {
            return candidate;
        }
        n += 1;
    }
}

/// 📋 Liste le contenu d'un dossier : dossiers d'abord (A→Z), puis fichiers (plus récents d'abord).
#[server]
pub async fn list_dir(path: String) -> Result<Vec<DriveEntry>, ServerFnError> {
    let dir = resolve_path(&path).map_err(ServerFnError::new)?;
    if path.trim_matches('/').is_empty() {
        tokio::fs::create_dir_all(&dir).await.map_err(ServerFnError::new)?;
    }

    let mut rd = tokio::fs::read_dir(&dir)
        .await
        .map_err(|_| ServerFnError::new("Dossier introuvable"))?;

    let mut out = Vec::new();
    while let Some(entry) = rd.next_entry().await.map_err(ServerFnError::new)? {
        let Ok(meta) = entry.metadata().await else { continue };
        let Some(name) = entry.file_name().to_str().map(|s| s.to_string()) else { continue };
        let modified = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        out.push(DriveEntry {
            name,
            is_dir: meta.is_dir(),
            size: if meta.is_dir() { 0 } else { meta.len() },
            modified,
        });
    }

    out.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then_with(|| {
            if a.is_dir {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            } else {
                b.modified.cmp(&a.modified)
            }
        })
    });
    Ok(out)
}

/// 📁 Crée un sous-dossier dans `parent`.
#[server]
pub async fn create_folder(parent: String, name: String) -> Result<(), ServerFnError> {
    let parent_dir = resolve_path(&parent).map_err(ServerFnError::new)?;
    let name = clean_name(&name).map_err(ServerFnError::new)?;
    if parent.trim_matches('/').is_empty() {
        tokio::fs::create_dir_all(&parent_dir).await.map_err(ServerFnError::new)?;
    }

    match tokio::fs::create_dir(parent_dir.join(&name)).await {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            Err(ServerFnError::new("Ce nom existe déjà"))
        }
        Err(e) => Err(ServerFnError::new(e)),
    }
}

/// 🗑️ Supprime un fichier, ou un dossier avec tout son contenu.
#[server]
pub async fn delete_entry(path: String) -> Result<(), ServerFnError> {
    if path.trim_matches('/').is_empty() {
        return Err(ServerFnError::new("Impossible de supprimer la racine"));
    }
    let target = resolve_path(&path).map_err(ServerFnError::new)?;
    let meta = tokio::fs::symlink_metadata(&target)
        .await
        .map_err(|_| ServerFnError::new("Élément introuvable"))?;

    if meta.is_dir() {
        tokio::fs::remove_dir_all(&target).await.map_err(ServerFnError::new)
    } else {
        tokio::fs::remove_file(&target).await.map_err(ServerFnError::new)
    }
}

/// ✏️ Renomme un fichier ou un dossier (reste dans le même dossier parent).
#[server]
pub async fn rename_entry(path: String, new_name: String) -> Result<(), ServerFnError> {
    if path.trim_matches('/').is_empty() {
        return Err(ServerFnError::new("Impossible de renommer la racine"));
    }
    let source = resolve_path(&path).map_err(ServerFnError::new)?;
    let new_name = clean_name(&new_name).map_err(ServerFnError::new)?;
    let dest = source
        .parent()
        .ok_or_else(|| ServerFnError::new("Chemin invalide"))?
        .join(&new_name);

    if tokio::fs::try_exists(&dest).await.unwrap_or(false) {
        return Err(ServerFnError::new("Ce nom existe déjà"));
    }
    tokio::fs::rename(&source, &dest)
        .await
        .map_err(|_| ServerFnError::new("Impossible de renommer"))
}
