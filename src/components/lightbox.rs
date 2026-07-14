use dioxus::prelude::*;
use crate::models::upload_state::PhotoFile;

#[component]
pub fn Lightbox(
    photos: Vec<String>,
    index: usize,
    on_close: EventHandler<()>,
    on_navigate: EventHandler<usize>,
) -> Element {
    let total = photos.len();
    let current = photos.get(index).cloned().unwrap_or_default();

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

            if PhotoFile::is_video(&current) {
                video {
                    src: "/uploads/{current}",
                    controls: true,
                    autoplay: true,
                    playsinline: true,
                    class: "max-h-[90vh] max-w-[90vw] object-contain rounded-lg shadow-2xl",
                    onclick: move |evt| evt.stop_propagation(),
                }
            } else {
                img {
                    src: "/uploads/{current}",
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
                href: "/uploads/{current}",
                download: "{current}",
                onclick: move |evt| evt.stop_propagation(),
                class: "absolute bottom-4 right-4 bg-white/90 rounded-full px-4 py-2 shadow-md text-sm font-semibold",
                "⬇️ Télécharger"
            }

            div { class: "absolute bottom-4 left-4 text-white/70 text-sm", "{index + 1} / {total}" }
        }
    }
}