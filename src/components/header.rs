use dioxus::prelude::*;
use crate::models::i18n::{t, LANG, Lang};

pub fn Header() -> Element {

    rsx! {
        div { class: "text-center",

            h1 { class: "text-5xl font-bold text-pink-600", "📸 {t(\"website_title\")}" }

            p { class: "mt-4 text-gray-600 text-lg", "{t(\"website_subtitle\")}" }
        }

        div { class: "flex gap-2 justify-center mb-4",
            button {
                class: if *LANG.read() == Lang::Fr { "font-bold underline" } else { "opacity-50" },
                onclick: move |_| *LANG.write() = Lang::Fr,
                "🇫🇷 FR"
            }
            button {
                class: if *LANG.read() == Lang::Pl { "font-bold underline" } else { "opacity-50" },
                onclick: move |_| *LANG.write() = Lang::Pl,
                "🇵🇱 PL"
            }

            button {
                class: if *LANG.read() == Lang::En { "font-bold underline" } else { "opacity-50" },
                onclick: move |_| *LANG.write() = Lang::En,
                "🇬🇧 EN"
            }
        }
    }
}