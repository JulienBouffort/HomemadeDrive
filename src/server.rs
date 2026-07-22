use dioxus::prelude::*;

#[cfg(feature = "server")]
use axum::extract::Multipart;
#[cfg(feature = "server")]
use tokio::io::AsyncWriteExt;

// 🚀 Route d'upload en streaming multipart (remplace l'ancien #[server])
#[cfg(feature = "server")]
pub async fn upload_photo_handler(
    mut multipart: Multipart,
) -> Result<axum::Json<String>, (axum::http::StatusCode, String)> {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let uploads_dir = std::path::Path::new(base_dir).join("uploads");
    tokio::fs::create_dir_all(&uploads_dir).await.ok();

    println!("👉 LE DOSSIER UPLOADS SE TROUVE ICI : {}", uploads_dir.display());

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let file_name = field.file_name().unwrap_or("photo.jpg").to_string();
        let unique_name = format!("{}-{}", uuid::Uuid::new_v4(), file_name);
        let file_path = uploads_dir.join(&unique_name);

        let mut file = tokio::fs::File::create(&file_path)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?
        {
            file.write_all(&chunk)
                .await
                .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }

        println!("📸 Image sauvegardée avec succès : {}", unique_name);
        return Ok(axum::Json(unique_name));
    }

    Err((axum::http::StatusCode::BAD_REQUEST, "Aucun fichier reçu".into()))
}

// 📤 Fonction CLIENT qui appelle la route ci-dessus via reqwest multipart
#[cfg(not(feature = "server"))]
pub async fn upload_photo(bytes: Vec<u8>, file_name: String) -> Result<String, String> {
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name(file_name)
        .mime_str("image/jpeg")
        .map_err(|e| e.to_string())?;

    let form = reqwest::multipart::Form::new().part("file", part);

    let res = reqwest::Client::new()
        .post("/api/upload")
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Échec de l'upload : {}", res.status()));
    }

    res.json::<String>().await.map_err(|e| e.to_string())
}

// 📋 Liste des photos déjà uploadées (server function, inchangée)
#[get("/api/photos")]
pub async fn list_photos() -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut entries = tokio::fs::read_dir("uploads")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
    {
        if let Some(name) = entry.file_name().to_str() {
            names.push(name.to_string());
        }
    }

    Ok(names)
}