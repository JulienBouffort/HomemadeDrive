use dioxus::prelude::*;


#[server]
// 🚀 On augmente la limite de taille d'Axum à 500 Mo pour cette fonction spécifique
#[middleware(axum::extract::DefaultBodyLimit::max(500 * 1024 * 1024))]
pub async fn upload_photo_server(bytes: Vec<u8>, file_name: String) -> Result<String, ServerFnError> {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let uploads_dir = std::path::Path::new(base_dir).join("uploads");
    
    tokio::fs::create_dir_all(&uploads_dir).await.ok();

    println!("👉 LE DOSSIER UPLOADS SE TROUVE ICI : {}", uploads_dir.display());

    let unique_name = format!("{}-{}", uuid::Uuid::new_v4(), file_name);
    let file_path = uploads_dir.join(&unique_name);

    tokio::fs::write(file_path, &bytes)
        .await
        .map_err(|e| ServerFnError::new(format!("Impossible d'écrire le fichier : {}", e)))?;

    println!("📸 Image sauvegardée avec succès : {}", unique_name);

    Ok(unique_name)
}

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