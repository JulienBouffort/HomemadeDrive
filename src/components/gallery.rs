use dioxus::prelude::*;
use crate::services::upload_services::list_photos;
use crate::components::lightbox::Lightbox;
use crate::models::upload_state::PhotoFile;
use crate::models::i18n::t;

pub static GALLERY_REFRESH: GlobalSignal<u32> = Signal::global(|| 0);


const PAGE_SIZE: usize = 12;

#[component]
pub fn Gallery() -> Element {
    let photos = use_resource(move || {
        let _ = GALLERY_REFRESH();
        async move { list_photos().await.unwrap_or_default() }
    });
    let mut visible_count = use_signal(|| PAGE_SIZE);
    let mut selected: Signal<Option<usize>> = use_signal(|| None);

    let all_photos = photos.read().clone().unwrap_or_default();
    let total = all_photos.len();
    let shown: Vec<String> = all_photos.iter().take(visible_count()).cloned().collect();

    rsx! {
        div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3",
            for (i , name) in shown.iter().enumerate() {
                div {
                    key: "{name}",
                    class: "relative group aspect-square overflow-hidden rounded-xl bg-gray-100 cursor-pointer",
                    onclick: move |_| selected.set(Some(i)),

                    if PhotoFile::is_video(name) {
                        video {
                            src: "/uploads/{name}",
                            muted: true,
                            preload: "metadata",
                            class: "w-full h-full object-cover transition-transform duration-200 group-hover:scale-105",
                        }
                        div { class: "absolute inset-0 flex items-center justify-center pointer-events-none",
                            div { class: "w-12 h-12 rounded-full bg-black/50 backdrop-blur-sm flex items-center justify-center text-white text-xl",
                                "▶"
                            }
                        }
                    } else {
                        img {
                            src: "/uploads/{name}",
                            loading: "lazy",
                            class: "w-full h-full object-cover transition-transform duration-200 group-hover:scale-105",
                        }
                    }
                    a {
                        href: "/uploads/{name}",
                        download: "{name}",
                        onclick: move |evt| evt.stop_propagation(),
                        class: "absolute bottom-2 right-2 bg-white/90 rounded-full p-2 shadow-md opacity-0 group-hover:opacity-100 transition-opacity",
                        "⬇️"
                    }
                }
            }
        }

        if visible_count() < total {
            div { class: "mt-6 flex justify-center",
                button {
                    class: "px-6 py-2.5 rounded-xl bg-rose-100 text-rose-700 font-semibold hover:bg-rose-200 transition-colors",
                    onclick: move |_| visible_count += PAGE_SIZE,
                    "{t(\"load_more\")} ({total - visible_count()} {t(\"remaining\")})"
                }
            }
        }

        if let Some(index) = selected() {
            Lightbox {
                photos: all_photos.clone(),
                index,
                on_close: move |_| selected.set(None),
                on_navigate: move |new_index| selected.set(Some(new_index)),
            }
        }
    }
}