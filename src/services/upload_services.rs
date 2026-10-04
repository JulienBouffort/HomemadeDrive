#[cfg(feature = "server")]
use axum::extract::Multipart;
#[cfg(feature = "server")]
use axum::http::StatusCode;
#[cfg(feature = "server")]
use tokio::io::AsyncWriteExt;

/// Paramètres de la requête : `/api/upload?path=dossier/sous-dossier`
#[cfg(feature = "server")]
#[derive(serde::Deserialize)]
pub struct UploadParams {
    #[serde(default)]
    pub path: String,
}

#[cfg(feature = "server")]
fn internal(e: impl ToString) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

#[cfg(feature = "server")]
fn bad_request(e: impl ToString) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, e.to_string())
}

// 🚀 ROUTE SERVEUR AXUM : Upload en streaming multipart dans le dossier `path`
#[cfg(feature = "server")]
pub async fn upload_photo_handler(
    axum::extract::Query(params): axum::extract::Query<UploadParams>,
    mut multipart: Multipart,
) -> Result<axum::Json<String>, (StatusCode, String)> {
    use crate::services::drive_services::{resolve_path, sanitize_upload_name, unique_path};

    let dir = resolve_path(&params.path).map_err(bad_request)?;
    if params.path.trim_matches('/').is_empty() {
        tokio::fs::create_dir_all(&dir).await.map_err(internal)?;
    } else if !tokio::fs::metadata(&dir).await.map(|m| m.is_dir()).unwrap_or(false) {
        return Err((StatusCode::NOT_FOUND, "Dossier de destination introuvable".into()));
    }

    while let Some(mut field) = multipart.next_field().await.map_err(bad_request)? {
        let raw_name = field.file_name().unwrap_or("fichier").to_string();
        let name = sanitize_upload_name(&raw_name);
        // Pas de doublon : "photo.jpg" -> "photo (1).jpg" si le nom existe déjà
        let file_path = unique_path(&dir, &name).await;
        let saved_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&name)
            .to_string();

        let written: Result<(), (StatusCode, String)> = async {
            let mut file = tokio::fs::File::create(&file_path).await.map_err(internal)?;
            while let Some(chunk) = field.chunk().await.map_err(bad_request)? {
                file.write_all(&chunk).await.map_err(internal)?;
            }
            file.flush().await.map_err(internal)?;
            Ok(())
        }
        .await;

        if let Err(e) = written {
            // On ne laisse pas un fichier à moitié écrit
            let _ = tokio::fs::remove_file(&file_path).await;
            return Err(e);
        }

        // 🔄 Conversion HEIC/HEIF -> JPEG si nécessaire (l'original est conservé)
        let lower_name = saved_name.to_lowercase();
        if lower_name.ends_with(".heic") || lower_name.ends_with(".heif") {
            let jpeg_path = unique_path(&dir, &format!("{}.jpg", saved_name)).await;

            let output = tokio::process::Command::new("heif-convert")
                .arg(&file_path)
                .arg(&jpeg_path)
                .output()
                .await;

            match output {
                Ok(o) if o.status.success() => {
                    let jpeg_name = jpeg_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&saved_name)
                        .to_string();
                    println!("🔄 Converti en JPEG : {}", jpeg_name);
                    return Ok(axum::Json(jpeg_name));
                }
                Ok(o) => {
                    println!("⚠️ Échec conversion HEIC : {}", String::from_utf8_lossy(&o.stderr));
                }
                Err(e) => {
                    println!("⚠️ heif-convert introuvable : {}", e);
                }
            }
        }

        println!("📁 Fichier sauvegardé : {}/{}", params.path, saved_name);
        return Ok(axum::Json(saved_name));
    }

    Err((StatusCode::BAD_REQUEST, "Aucun fichier reçu".into()))
}

/// 📤 Envoie un fichier (n'importe quel type) dans le dossier `dir` du drive.
pub async fn upload_photo(bytes: Vec<u8>, file_name: String, dir: String) -> Result<String, String> {
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name(file_name)
        .mime_str("application/octet-stream")
        .map_err(|e| e.to_string())?;

    let form = reqwest::multipart::Form::new().part("file", part);

    // 💡 Récupération dynamique de l'URL
    let mut api_url = "/api/upload".to_string();

    #[cfg(not(feature = "server"))]
    {
        if let Some(window) = web_sys::window() {
            let location = window.location();
            if let Ok(origin) = location.origin() {
                api_url = format!("{}{}", origin, "/api/upload");
            }
        }
    }

    let res = reqwest::Client::new()
        .post(&api_url)
        .query(&[("path", dir)])
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Erreur réseau : {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Échec de l'upload : {}", res.status()));
    }

    res.json::<String>().await.map_err(|e| format!("Erreur JSON : {}", e))
}
