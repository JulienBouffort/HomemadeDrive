use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
// On importe la fonction serveur que nous avons créée ensemble
use crate::services::upload_services::upload_photo_server; 
use crate::models::upload_state::{PhotoFile, UploadStatus};
use crate::models::i18n::{t, LANG, Lang};
use crate::components::{
    file_list::FileList,
    header::Header,
    status::Status,
    upload_zone::UploadZone,
    gallery::{Gallery, GALLERY_REFRESH},
};
pub fn App() -> Element {
    let mut photos = use_signal(Vec::<PhotoFile>::new);

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(lang) = window.navigator().language() {
                    let detected = if lang.to_lowercase().starts_with("pl") {
                        Lang::Pl
                    } else {
                        Lang::Fr
                    };
                    *LANG.write() = detected;
                }
            }
        }
    });

    // Calcul du nombre de photos prêtes
    let ready_count = photos.read()
        .iter()
        .filter(|p| p.status == UploadStatus::Idle) // ou votre statut d'attente
        .count();

    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        div { class: "min-h-screen bg-gradient-to-tr from-rose-100 via-stone-50 to-teal-50 flex flex-col lg:flex-row items-center justify-center gap-6 p-4 md:p-8 font-sans",
            div { class: "bg-white/80 backdrop-blur-md shadow-2xl rounded-3xl p-6 md:p-10 w-full max-w-2xl border border-white/20",

                Header {}

                // La zone de drop interactive
                UploadZone { photos }

                // Liste stylisée sous forme de grille moderne
                FileList { photos }

                // Statut global et bouton d'action
                if !photos.read().is_empty() {
                    div { class: "mt-8 pt-6 border-t border-gray-100 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4",
                        span { class: "text-gray-600 font-medium text-center sm:text-left",
                            "✨ {ready_count} photo(s) prête(s) à être partagée(s)"
                        }
                        button {
                            class: "px-8 py-3.5 rounded-2xl bg-gradient-to-r from-rose-500 to-pink-500 text-white font-bold shadow-lg shadow-pink-500/20 hover:shadow-pink-500/30 hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 flex items-center justify-center gap-2",
                            onclick: move |_| {
                                // On lance une tâche asynchrone pour gérer l'envoi
                                spawn(async move {
                                    // 1. On extrait la liste des photos prêtes à être envoyées
                                    let pending_photos: Vec<(String, Vec<u8>)> = photos
                                        .read()
                                        .iter()
                                        .filter(|p| p.status == UploadStatus::Idle && p.bytes.is_some())
                                        .map(|p| (p.name.clone(), p.bytes.as_ref().unwrap().clone())) // 2. On passe le statut de l'image à "Uploading"
                                        .collect();
                                    println!("📸 {} photo(s) prête(s) à l'envoi", pending_photos.len());
                                    for (name, bytes) in pending_photos {
                                        if let Some(photo) = photos.write().iter_mut().find(|p| p.name == name) {
                                            photo.status = UploadStatus::Uploading; // Optionnel : On vide les bytes de la mémoire car le fichier est sur le serveur
                                        }
                                        match upload_photo_server(bytes, name.clone()).await {
                                            Ok(unique_name) => {
                                                println!("✅ Enregistré sous : {}", unique_name);
                                                if let Some(photo) = photos
                                                    .write()
                                                    .iter_mut()
                                                    .find(|p| p.name == name)
                                                {
                                                    photo.status = UploadStatus::Completed;
                                                    photo.bytes = None;
                                                }
                                            }
                                            Err(err) => {
                                                println!("❌ Erreur d'upload pour {} : {:?}", name, err);
                                                if let Some(photo) = photos
                                                    .write()
                                                    .iter_mut()
                                                    .find(|p| p.name == name)
                                                {
                                                    photo.status = UploadStatus::Error;
                                                }
                                            }
                                        }
                                    }
                                    *GALLERY_REFRESH.write() += 1; // On déclenche le rafraîchissement de la galerie
                                });
                            },
                            span { "🚀" }
                            span { "{t(\"share_button\")}" }
                        }
                    }
                }
            }
            // --- Carte 2 : Galerie / album partagé ---
            div { class: "bg-white/80 backdrop-blur-md shadow-2xl rounded-3xl p-6 md:p-10 w-full max-w-4xl border border-white/20",
                h2 { class: "text-2xl font-bold text-gray-800 mb-1 flex items-center gap-2",
                    span { "🖼️" }
                    "{t(\"gallery_title\")}"
                }
                p { class: "text-sm text-gray-500 mb-6", "{t(\"gallery_subtitle\")}" }

                Gallery {}
            }
        }
    }
}
    

