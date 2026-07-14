use dioxus::prelude::*;
use crate::models::upload_state::{PhotoFile, UploadStatus};
// On importe la fonction serveur que l'on vient de créer
use crate::services::upload_services::upload_photo_server; 

use crate::models::i18n::t;


#[component]
pub fn UploadZone(mut photos: Signal<Vec<PhotoFile>>) -> Element {
    rsx! {
        label { class: "group mt-6 flex flex-col items-center justify-center w-full h-48 border-2 border-dashed border-rose-200 hover:border-rose-400 rounded-2xl cursor-pointer bg-rose-50/30 hover:bg-rose-50/60 transition-all duration-300 p-6 text-center",

            // Icône animée au survol
            div { class: "p-3 bg-white rounded-full shadow-md text-rose-500 group-hover:scale-110 transition-transform duration-300",
                svg {
                    class: "w-8 h-8",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "0 0 24 24 M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
                    }
                }
            }

            span { class: "mt-4 text-sm font-semibold text-gray-700", "{t(\"drag_drop\")}" }
            span { class: "mt-1 text-xs text-gray-500", "{t(\"browse_files\")}" }

            input {
                r#type: "file",
                class: "hidden", // On cache l'input moche du navigateur
                multiple: true,
                accept: "image/*,video/*",
                onchange: move |evt| {
                    let files = evt.files();

                    let mut list = Vec::new();
                    for file in &files {
                        list.push(PhotoFile::new(file.name(), file.size() as usize));
                    }
                    photos.set(list);

                    // Dans upload_zone.rs, lors de l'événement onchange / ondrop :
                    for file in files {
                        spawn(async move {
                            let file_name = file.name();

                            // 1. On lit les octets localement dans le navigateur
                            if let Ok(bytes) = file.read_bytes().await {
                                // 2. On enregistre les octets dans le signal pour le bouton
                                if let Some(photo) = photos
                                    .write() // Prêt à être envoyé
                                    .iter_mut()
                                    .find(|p| p.name == file_name)
                                {
                                    photo.bytes = Some(bytes.to_vec());
                                    photo.status = UploadStatus::Idle;
                                }
                            }
                        });
                    }
                },
            }
        }
    }
}