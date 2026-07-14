use dioxus::prelude::*;

mod app;
mod components;
mod models;
mod services;



fn main() {
    #[cfg(feature = "server")]
    {
        use tower_http::services::ServeDir;

        dioxus::serve(|| async move {
            let router = dioxus::server::router(app::App)
                .nest_service("/uploads", ServeDir::new("uploads"));

            Ok(router)
        });
    }

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);
}