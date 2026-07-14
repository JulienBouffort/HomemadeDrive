use dioxus::prelude::*;

mod app;
mod components;
mod models;
mod services;

const UPLOADS_DIR: &str = r"D:\uploads";

fn main() {
    #[cfg(feature = "server")]
    {
        use tower_http::services::ServeDir;

        dioxus::serve(|| async move {
            let router = dioxus::server::router(app::App)
                .nest_service("/uploads", ServeDir::new(UPLOADS_DIR));

            Ok(router)
        });
    }

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);
}