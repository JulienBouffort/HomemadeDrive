use serde::{Deserialize, Serialize};

/// Une entrée (fichier ou dossier) renvoyée par le serveur.
/// `name` est le vrai nom sur le disque (sans chemin).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DriveEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    /// Date de dernière modification (secondes depuis l'epoch Unix)
    pub modified: u64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FileKind {
    Folder,
    Image,
    Video,
    Audio,
    Pdf,
    Archive,
    Text,
    Other,
}

impl FileKind {
    pub fn icon(self) -> &'static str {
        match self {
            FileKind::Folder => "📁",
            FileKind::Image => "🖼️",
            FileKind::Video => "🎬",
            FileKind::Audio => "🎵",
            FileKind::Pdf => "📕",
            FileKind::Archive => "🗜️",
            FileKind::Text => "📝",
            FileKind::Other => "📄",
        }
    }
}

impl DriveEntry {
    pub fn kind(&self) -> FileKind {
        if self.is_dir {
            return FileKind::Folder;
        }
        let ext = self.name.rsplit('.').next().unwrap_or("").to_lowercase();
        match ext.as_str() {
            // HEIC/HEIF volontairement exclus : la plupart des navigateurs ne savent pas les afficher
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" | "avif" => FileKind::Image,
            "mp4" | "mov" | "webm" | "avi" | "mkv" | "m4v" => FileKind::Video,
            "mp3" | "wav" | "ogg" | "m4a" | "flac" | "aac" => FileKind::Audio,
            "pdf" => FileKind::Pdf,
            "zip" | "7z" | "rar" | "tar" | "gz" => FileKind::Archive,
            "txt" | "md" | "doc" | "docx" | "odt" | "rtf" | "csv" | "xls" | "xlsx" => FileKind::Text,
            _ => FileKind::Other,
        }
    }

    pub fn is_media(&self) -> bool {
        matches!(self.kind(), FileKind::Image | FileKind::Video)
    }

    /// Nom affiché à l'utilisateur (préfixe UUID des anciens uploads retiré).
    pub fn display_name(&self) -> String {
        display_name(&self.name)
    }
}

/// Retire le préfixe `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx-` ajouté par l'ancienne version.
pub fn display_name(raw: &str) -> String {
    if let Some(prefix) = raw.get(..36) {
        let b = prefix.as_bytes();
        let looks_like_uuid = b.iter().enumerate().all(|(i, c)| match i {
            8 | 13 | 18 | 23 => *c == b'-',
            _ => c.is_ascii_hexdigit(),
        });
        if looks_like_uuid && raw.as_bytes().get(36) == Some(&b'-') && raw.len() > 37 {
            return raw[37..].to_string();
        }
    }
    raw.to_string()
}

/// Dernier segment d'un chemin relatif ("a/b/c.jpg" -> "c.jpg").
pub fn file_name_of(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// Assemble un dossier et un nom ("" + "a" -> "a", "a" + "b" -> "a/b").
pub fn join_path(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_string()
    } else {
        format!("{}/{}", dir.trim_end_matches('/'), name)
    }
}

/// Fil d'Ariane : "a/b/c" -> [("a","a"), ("b","a/b"), ("c","a/b/c")]
pub fn breadcrumbs(dir: &str) -> Vec<(String, String)> {
    let mut acc = String::new();
    let mut out = Vec::new();
    for part in dir.split('/').filter(|s| !s.is_empty()) {
        acc = join_path(&acc, part);
        out.push((part.to_string(), acc.clone()));
    }
    out
}

/// Encode chaque segment d'un chemin pour l'utiliser dans une URL.
pub fn encode_path(path: &str) -> String {
    path.split('/')
        .map(|seg| {
            let mut out = String::new();
            for byte in seg.bytes() {
                match byte {
                    b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                        out.push(byte as char)
                    }
                    _ => out.push_str(&format!("%{:02X}", byte)),
                }
            }
            out
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// URL publique d'un fichier à partir de son chemin relatif.
pub fn file_url(path: &str) -> String {
    format!("/uploads/{}", encode_path(path))
}

pub fn human_size(bytes: u64) -> String {
    const KO: f64 = 1024.0;
    let b = bytes as f64;
    if b >= KO * KO * KO {
        format!("{:.1} Go", b / (KO * KO * KO))
    } else if b >= KO * KO {
        format!("{:.1} Mo", b / (KO * KO))
    } else {
        format!("{:.0} Ko", (b / KO).max(1.0))
    }
}
