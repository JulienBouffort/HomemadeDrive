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
        ("share_button", Lang::Fr) => "Ajouter des fichiers",
        ("share_button", Lang::Pl) => "Dodaj pliki",
        ("share_button", Lang::En) => "Add files",

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

        ("drag_drop", Lang::Fr) => "Cliquez ou glissez vos fichiers ici",
        ("drag_drop", Lang::Pl) => "Kliknij lub przeciągnij swoje pliki tutaj",
        ("drag_drop", Lang::En) => "Drag and drop your files here",

        ("browse_files", Lang::Fr) => "ou cliquez pour parcourir vos fichiers",
        ("browse_files", Lang::Pl) => "lub kliknij, aby przeglądać pliki",
        ("browse_files", Lang::En) => "or click to browse your files",

        ("gallery_title", Lang::Fr) => "Dossiers",
        ("gallery_title", Lang::Pl) => "Dokumenty",
        ("gallery_title", Lang::En) => "Folders",

        ("website_title", Lang::Fr) => "Serveur Famille Bouffort/Łagosz",
        ("website_title", Lang::Pl) => "Serwer rodziny Bouffort/Łagosz",
        ("website_title", Lang::En) => "Wedding Album",

        ("website_subtitle", Lang::Fr) => "Documents, photos et tout ce dont nous avons besoin",
        ("website_subtitle", Lang::Pl) => "Dokumenty, zdjęcia i wszystko, czego potrzebujemy",
        ("website_subtitle", Lang::En) => {
            "Share your most beautiful memories with the newlyweds ❤️"
        }

        ("gallery_subtitle", Lang::Fr) => "Toutes les documents partagés",
        ("gallery_subtitle", Lang::Pl) => "Wszystkie zdjęcia udostępnione przez Twoich gości",
        ("gallery_subtitle", Lang::En) => "All photos shared by your guests",

        ("load_more", Lang::Fr) => "Charger plus de photos",
        ("load_more", Lang::Pl) => "Załaduj więcej zdjęć",
        ("load_more", Lang::En) => "Load more photos",

        ("remaining", Lang::Fr) => "restantes",
        ("remaining", Lang::Pl) => "pozostało",
        ("remaining", Lang::En) => "remaining",

        ("new_folder", Lang::Fr) => "Nouveau dossier",
        ("new_folder", Lang::Pl) => "Nowy folder",
        ("new_folder", Lang::En) => "New folder",

        ("folder_name", Lang::Fr) => "Nom du dossier",
        ("folder_name", Lang::Pl) => "Nazwa folderu",
        ("folder_name", Lang::En) => "Folder name",

        ("new_name", Lang::Fr) => "Nouveau nom",
        ("new_name", Lang::Pl) => "Nowa nazwa",
        ("new_name", Lang::En) => "New name",

        ("rename", Lang::Fr) => "Renommer",
        ("rename", Lang::Pl) => "Zmień nazwę",
        ("rename", Lang::En) => "Rename",

        ("delete", Lang::Fr) => "Supprimer",
        ("delete", Lang::Pl) => "Usuń",
        ("delete", Lang::En) => "Delete",

        ("cancel", Lang::Fr) => "Annuler",
        ("cancel", Lang::Pl) => "Anuluj",
        ("cancel", Lang::En) => "Cancel",

        ("create", Lang::Fr) => "Créer",
        ("create", Lang::Pl) => "Utwórz",
        ("create", Lang::En) => "Create",

        ("open", Lang::Fr) => "Ouvrir",
        ("open", Lang::Pl) => "Otwórz",
        ("open", Lang::En) => "Open",

        ("download", Lang::Fr) => "Télécharger",
        ("download", Lang::Pl) => "Pobierz",
        ("download", Lang::En) => "Download",

        ("confirm_delete_file", Lang::Fr) => "Supprimer définitivement ce fichier ?",
        ("confirm_delete_file", Lang::Pl) => "Usunąć ten plik na stałe?",
        ("confirm_delete_file", Lang::En) => "Permanently delete this file?",

        ("confirm_delete_folder", Lang::Fr) => "Supprimer ce dossier et tout son contenu ?",
        ("confirm_delete_folder", Lang::Pl) => "Usunąć ten folder wraz z całą zawartością?",
        ("confirm_delete_folder", Lang::En) => "Delete this folder and all its contents?",

        ("root", Lang::Fr) => "Racine",
        ("root", Lang::Pl) => "Katalog główny",
        ("root", Lang::En) => "Root",

        ("empty_folder", Lang::Fr) => "Ce dossier est vide",
        ("empty_folder", Lang::Pl) => "Ten folder jest pusty",
        ("empty_folder", Lang::En) => "This folder is empty",

        ("loading", Lang::Fr) => "Chargement...",
        ("loading", Lang::Pl) => "Ładowanie...",
        ("loading", Lang::En) => "Loading...",

        ("upload_to", Lang::Fr) => "Envoi vers :",
        ("upload_to", Lang::Pl) => "Wyślij do:",
        ("upload_to", Lang::En) => "Upload to:",

        ("selected_files", Lang::Fr) => "Fichiers sélectionnés",
        ("selected_files", Lang::Pl) => "Wybrane pliki",
        ("selected_files", Lang::En) => "Selected files",

        ("files_ready", Lang::Fr) => "fichier(s) prêt(s) à être partagé(s)",
        ("files_ready", Lang::Pl) => "plików gotowych do udostępnienia",
        ("files_ready", Lang::En) => "file(s) ready to be shared",

        _ => key,
    }
    .to_string()
}
