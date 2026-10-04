use dioxus::prelude::*;
use crate::models::drive::{display_name, file_name_of, file_url, DriveEntry, FileKind};
use crate::models::i18n::t;

/// Visionneuse plein écran. `photos` contient les chemins relatifs (ex: "Mariage/IMG_1.jpg")
/// des images et vidéos du dossier courant.
#[component]
pub fn Lightbox(
    photos: Vec<String>,
    index: usize,
    on_close: EventHandler<()>,
    on_navigate: EventHandler<usize>,
) -> Element {
    let total = photos.len();
    let current = photos.get(index).cloned().unwrap_or_default();
    let url = file_url(&current);
    let name = display_name(file_name_of(&current));
    let is_video = DriveEntry {
        name: file_name_of(&current).to_string(),
        is_dir: false,
        size: 0,
        modified: 0,
    }
    .kind()
        == FileKind::Video;
    let download_label = t("download_button");

    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/90 backdrop-blur-sm flex items-center justify-center p-4",
            onclick: move |_| on_close.call(()),

            button {
                class: "absolute top-4 right-4 text-white text-3xl hover:text-rose-300 transition-colors",
                onclick: move |evt| {
                    evt.stop_propagation();
                    on_close.call(());
                },
                "✕"
            }

            div { class: "absolute top-4 left-4 right-16 text-white/80 text-sm truncate", "{name}" }

            if index > 0 {
                button {
                    class: "absolute left-4 top-1/2 -translate-y-1/2 w-11 h-11 flex items-center justify-center rounded-full bg-black/40 hover:bg-black/60 backdrop-blur-sm text-white text-2xl shadow-lg transition-colors",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_navigate.call(index - 1);
                    },
                    "‹"
                }
            }

            if is_video {
                video {
                    src: "{url}",
                    controls: true,
                    autoplay: true,
                    playsinline: true,
                    class: "max-h-[90vh] max-w-[90vw] object-contain rounded-lg shadow-2xl",
                    onclick: move |evt| evt.stop_propagation(),
                }
            } else {
                img {
                    src: "{url}",
                    class: "max-h-[90vh] max-w-[90vw] object-contain rounded-lg shadow-2xl",
                    onclick: move |evt| evt.stop_propagation(),
                }
            }

            if index + 1 < total {
                button {
                    class: "absolute right-4 top-1/2 -translate-y-1/2 w-11 h-11 flex items-center justify-center rounded-full bg-black/40 hover:bg-black/60 backdrop-blur-sm text-white text-2xl shadow-lg transition-colors",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_navigate.call(index + 1);
                    },
                    "›"
                }
            }

            a {
                href: "{url}",
                download: "{name}",
                onclick: move |evt| evt.stop_propagation(),
                class: "absolute bottom-4 right-4 bg-white/90 rounded-full px-4 py-2 shadow-md text-sm font-semibold",
                "{download_label}"
            }

            div { class: "absolute bottom-4 left-4 text-white/70 text-sm", "{index + 1} / {total}" }
        }
    }
}
