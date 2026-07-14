use dioxus::prelude::*;
use crate::models::upload_state::{PhotoFile, UploadStatus};

// Fonction utilitaire pour formater la taille
fn format_size(bytes: usize) -> String {
    if bytes > 1024 * 1024 {
        format!("{:.1} Mo", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.0} Ko", bytes as f64 / 1024.0)
    }
}

#[component]
pub fn FileList(photos: Signal<Vec<PhotoFile>>) -> Element {
    let list = photos.read();
    
    if list.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "mt-8",
            h3 { class: "text-xs font-bold text-gray-400 uppercase tracking-wider mb-3",
                "Photos sélectionnées"
            }

            // Grille CSS fluide
            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-3 max-h-64 overflow-y-auto pr-1 custom-scrollbar",
                for photo in list.iter() {
                    div { class: "flex items-center justify-between p-3 bg-gray-50 rounded-xl border border-gray-100 hover:border-gray-200 transition-colors",

                        // Infos du fichier (Icône + Nom + Taille)
                        div { class: "flex items-center gap-3 min-w-0",
                            div { class: "p-2 bg-rose-100 text-rose-600 rounded-lg shrink-0",
                                "📸"
                            }
                            div { class: "min-w-0",
                                p { class: "text-sm font-semibold text-gray-700 truncate pr-2",
                                    "{photo.name}"
                                }
                                p { class: "text-xs text-gray-400", "{format_size(photo.size)}" }
                            }
                        }

                        match photo.status {
                            // 🚀 On gère Idle OU Ready avec la même ligne
                            UploadStatus::Idle | UploadStatus::Ready => rsx! {
                                span { class: "px-2.5 py-1 text-xs font-medium rounded-full bg-blue-50 text-blue-600 border border-blue-100 shrink-0",
                                    "Prêt"
                                }
                            },
                            UploadStatus::Uploading => rsx! {
                                span { class: "px-2.5 py-1 text-xs font-medium rounded-full bg-amber-50 text-amber-600 border border-amber-100 animate-pulse shrink-0",
                                    "Envoi..."
                                }
                            },
                            UploadStatus::Completed => rsx! {
                                span { class: "px-2.5 py-1 text-xs font-medium rounded-full bg-emerald-50 text-emerald-600 border border-emerald-100 shrink-0",
                                    "Enregistré ✓"
                                }
                            },
                            UploadStatus::Error => rsx! {
                                span { class: "px-2.5 py-1 text-xs font-medium rounded-full bg-rose-50 text-rose-600 border border-rose-100 shrink-0",
                                    "Erreur ❌"
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}