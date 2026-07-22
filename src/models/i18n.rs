use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Lang {
    Fr,
    Pl,
    En,
}

pub static LANG: GlobalSignal<Lang> = Signal::global(|| Lang::Fr);

pub fn t(key: &str) -> String {
    let lang = *LANG.read();
    match (key, lang) {
        ("share_button", Lang::Fr) => "Ajouter des photos et vidéos",
        ("share_button", Lang::Pl) => "Dodaj zdjęcia i filmy",
        ("share_button", Lang::En) => "Add photos and videos",

        ("ready_state", Lang::Fr) => "Prêt",
        ("ready_state", Lang::Pl) => "Gotowe",
        ("ready_state", Lang::En) => "Ready",

        ("uploading_state", Lang::Fr) => "Envoi...",
        ("uploading_state", Lang::Pl) => "Trwa przesyłanie...",
        ("uploading_state", Lang::En) => "Uploading...",

        ("completed_state", Lang::Fr) => "Enregistré ✓",
        ("completed_state", Lang::Pl) => "Zapisane ✓",
        ("completed_state", Lang::En) => "Uploaded ✓",

        ("error_state", Lang::Fr) => "Erreur ❌",
        ("error_state", Lang::Pl) => "Błąd ❌",
        ("error_state", Lang::En) => "Error ❌",

        ("download_button", Lang::Fr) => "⬇️ Télécharger",
        ("download_button", Lang::Pl) => "⬇️ Pobierz",
        ("download_button", Lang::En) => "⬇️ Download",

        ("ready_count", Lang::Fr) => "photo(s) prête(s) à être partagée(s)",
        ("ready_count", Lang::Pl) => "zdjęć gotowych do udostępnienia",
        ("ready_count", Lang::En) => "photo(s) ready to be shared",

        ("drag_drop", Lang::Fr) => "Cliquez ou Glissez vos photos et vidéos ici",
        ("drag_drop", Lang::Pl) => "Kliknij lub przeciągnij swoje zdjęcia i filmy tutaj",
        ("drag_drop", Lang::En) => "Drag and drop your photos and videos here",

        ("browse_files", Lang::Fr) => "ou cliquez pour parcourir vos fichiers",
        ("browse_files", Lang::Pl) => "lub kliknij, aby przeglądać pliki",
        ("browse_files", Lang::En) => "or click to browse your files",

        ("gallery_title", Lang::Fr) => "L'album des souvenirs",
        ("gallery_title", Lang::Pl) => "Album wspomnień",
        ("gallery_title", Lang::En) => "The Memory Album",

        ("website_title", Lang::Fr) => "Album du mariage",
        ("website_title", Lang::Pl) => "Album ślubny",
        ("website_title", Lang::En) => "Wedding Album",

        ("website_subtitle", Lang::Fr) => "Partagez vos plus beaux souvenirs avec les mariés ❤️",
        ("website_subtitle", Lang::Pl) => "Udostępnij swoje najpiękniejsze wspomnienia z wesela ❤️",
        ("website_subtitle", Lang::En) => "Share your most beautiful memories with the newlyweds ❤️",

        ("gallery_subtitle", Lang::Fr) => "Toutes les photos partagées par vos invités",
        ("gallery_subtitle", Lang::Pl) => "Wszystkie zdjęcia udostępnione przez Twoich gości",
        ("gallery_subtitle", Lang::En) => "All photos shared by your guests",

        ("load_more", Lang::Fr) => "Charger plus de photos",
        ("load_more", Lang::Pl) => "Załaduj więcej zdjęć",
        ("load_more", Lang::En) => "Load more photos",

        ("remaining", Lang::Fr) => "restantes",
        ("remaining", Lang::Pl) => "pozostało",
        ("remaining", Lang::En) => "remaining",

        _ => key,
    }
    .to_string()
}