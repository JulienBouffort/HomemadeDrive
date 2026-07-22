use dioxus::prelude::*;

mod app;
mod components;
mod models;
mod services;

fn main() {
    // 🖥️ CONFIGURATION CÔTÉ SERVEUR
    #[cfg(feature = "server")]
    {
        use tower_http::services::ServeDir;
        use tower_http::cors::{Any, CorsLayer};
        use axum::http::Method;

        dioxus::serve(|| async move {
            // Configuration du CORS pour autoriser ton smartphone (via le tunnel) à communiquer avec l'API Axum
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any);

            let router = dioxus::server::router(app::App)
                .nest_service("/uploads", ServeDir::new("uploads"))
                .route("/api/upload", axum::routing::post(services::upload_services::upload_photo_handler))
                .layer(axum::extract::DefaultBodyLimit::max(500 * 1024 * 1024))
                .layer(cors); // 💡 On applique le CORS ici

            Ok(router)
        });
    }

    // 📱 CONFIGURATION CÔTÉ CLIENT (Navigateur / Smartphone)
    #[cfg(not(feature = "server"))]
    {
        // Optionnel : Si Dioxus s'emmêle les pinceaux avec ses fonctions #[server] en passant par le tunnel,
        // tu peux décommenter la ligne ci-dessous et y coller l'adresse HTTPS de ton tunnel Cloudflare.
        // dioxus::server::prelude::set_server_url("https://ton-tunnel-cloudflare.trycloudflare.com");

        dioxus::launch(app::App);
    }
}