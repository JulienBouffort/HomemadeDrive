use dioxus::prelude::*;
use crate::components::lightbox::Lightbox;
use crate::models::drive::{breadcrumbs, file_url, human_size, join_path, DriveEntry, FileKind};
use crate::models::i18n::t;
use crate::services::drive_services::{create_folder, delete_entry, list_dir, rename_entry};

/// Incrémenter pour recharger le contenu du dossier courant.
pub static GALLERY_REFRESH: GlobalSignal<u32> = Signal::global(|| 0);
/// Dossier courant, relatif à `uploads/` ("" = racine). Partagé avec la zone d'upload.
pub static CURRENT_DIR: GlobalSignal<String> = Signal::global(|| String::new());

const PAGE_SIZE: usize = 12;

#[derive(Clone, PartialEq)]
enum Dialog {
    None,
    NewFolder,
    Rename(DriveEntry),
    Delete(DriveEntry),
}

#[component]
pub fn Gallery() -> Element {
    let entries = use_resource(move || {
        let _ = GALLERY_REFRESH();
        let dir = CURRENT_DIR();
        async move { list_dir(dir).await }
    });

    let mut visible_count = use_signal(|| PAGE_SIZE);
    let mut selected: Signal<Option<usize>> = use_signal(|| None);
    let mut dialog = use_signal(|| Dialog::None);
    let mut dialog_input = use_signal(String::new);
    let mut dialog_error: Signal<Option<String>> = use_signal(|| None);

    let dir = CURRENT_DIR();

    // Changement de dossier : on repart de zéro (pagination, visionneuse)
    let mut go_to = move |path: String| {
        *CURRENT_DIR.write() = path;
        visible_count.set(PAGE_SIZE);
        selected.set(None);
    };

    // Valide la boîte de dialogue en cours (création, renommage ou suppression)
    let mut submit_dialog = move || {
        let current = dialog();
        let value = dialog_input();
        let dir = CURRENT_DIR();
        spawn(async move {
            let result = match current {
                Dialog::NewFolder => create_folder(dir, value).await,
                Dialog::Rename(e) => rename_entry(join_path(&dir, &e.name), value).await,
                Dialog::Delete(e) => delete_entry(join_path(&dir, &e.name)).await,
                Dialog::None => Ok(()),
            };
            match result {
                Ok(()) => {
                    dialog.set(Dialog::None);
                    dialog_error.set(None);
                    *GALLERY_REFRESH.write() += 1;
                }
                Err(e) => dialog_error.set(Some(e.to_string())),
            }
        });
    };

    // Résultat de la liste (None = chargement en cours)
    let loaded: Option<Result<Vec<DriveEntry>, String>> = match &*entries.read() {
        None => None,
        Some(Ok(v)) => Some(Ok(v.clone())),
        Some(Err(e)) => Some(Err(e.to_string())),
    };
    let is_loading = loaded.is_none();
    let load_error = match &loaded {
        Some(Err(e)) => Some(e.clone()),
        _ => None,
    };
    let items = match loaded {
        Some(Ok(v)) => v,
        _ => Vec::new(),
    };

    // Chaque entrée est associée à son index dans la visionneuse (images/vidéos uniquement)
    let mut media_paths: Vec<String> = Vec::new();
    let mut rows: Vec<(DriveEntry, Option<usize>)> = Vec::new();
    for entry in items {
        let media_index = if entry.is_media() {
            media_paths.push(join_path(&dir, &entry.name));
            Some(media_paths.len() - 1)
        } else {
            None
        };
        rows.push((entry, media_index));
    }
    let total = rows.len();
    let shown: Vec<(DriveEntry, Option<usize>)> = rows.into_iter().take(visible_count()).collect();

    // Fil d'Ariane
    let mut crumbs = vec![(format!("🏠 {}", t("root")), String::new())];
    crumbs.extend(breadcrumbs(&dir));

    // Boîte de dialogue
    let current_dialog = dialog();
    let dialog_open = current_dialog != Dialog::None;
    let needs_input = matches!(current_dialog, Dialog::NewFolder | Dialog::Rename(_));
    let is_delete = matches!(current_dialog, Dialog::Delete(_));
    let dialog_title = match &current_dialog {
        Dialog::NewFolder => t("new_folder"),
        Dialog::Rename(_) => t("rename"),
        Dialog::Delete(_) => t("delete"),
        Dialog::None => String::new(),
    };
    let dialog_text = match &current_dialog {
        Dialog::Delete(e) => format!(
            "« {} » — {}",
            e.display_name(),
            if e.is_dir { t("confirm_delete_folder") } else { t("confirm_delete_file") }
        ),
        _ => String::new(),
    };
    let input_placeholder = match &current_dialog {
        Dialog::NewFolder => t("folder_name"),
        _ => t("new_name"),
    };
    let confirm_label = match &current_dialog {
        Dialog::NewFolder => t("create"),
        Dialog::Rename(_) => t("rename"),
        Dialog::Delete(_) => t("delete"),
        Dialog::None => String::new(),
    };
    let confirm_class = if is_delete {
        "px-4 py-2 rounded-xl bg-rose-600 text-white font-semibold hover:bg-rose-700 transition-colors"
    } else {
        "px-4 py-2 rounded-xl bg-pink-500 text-white font-semibold hover:bg-pink-600 transition-colors"
    };

    let new_folder_label = t("new_folder");
    let cancel_label = t("cancel");
    let empty_label = t("empty_folder");
    let loading_label = t("loading");

    rsx! {
        // ── Barre d'outils : fil d'Ariane + nouveau dossier ──
        div { class: "flex flex-wrap items-center justify-between gap-3 mb-4",
            nav { class: "flex flex-wrap items-center gap-1 text-sm text-gray-600",
                for (i , (label , path)) in crumbs.into_iter().enumerate() {
                    if i > 0 {
                        span { class: "text-gray-300", "/" }
                    }
                    button {
                        key: "{path}",
                        class: "px-2 py-1 rounded-lg hover:bg-rose-50 hover:text-rose-700 transition-colors font-medium",
                        onclick: move |_| go_to(path.clone()),
                        "{label}"
                    }
                }
            }
            button {
                class: "px-4 py-2 rounded-xl bg-rose-100 text-rose-700 font-semibold hover:bg-rose-200 transition-colors text-sm",
                onclick: move |_| {
                    dialog_input.set(String::new());
                    dialog_error.set(None);
                    dialog.set(Dialog::NewFolder);
                },
                "📁 {new_folder_label}"
            }
        }

        if let Some(err) = load_error {
            p { class: "text-sm text-rose-600 mb-4", "{err}" }
        }

        if is_loading {
            p { class: "text-sm text-gray-400 py-8 text-center", "{loading_label}" }
        } else if total == 0 {
            p { class: "text-sm text-gray-400 py-8 text-center", "{empty_label}" }
        }

        // ── Contenu du dossier ──
        div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3",
            for (entry , media_index) in shown {
                EntryTile {
                    key: "{entry.name}",
                    entry: entry.clone(),
                    dir: dir.clone(),
                    media_index,
                    on_open_folder: move |path: String| go_to(path),
                    on_open_media: move |index: usize| selected.set(Some(index)),
                    on_rename: move |e: DriveEntry| {
                        dialog_input.set(e.display_name());
                        dialog_error.set(None);
                        dialog.set(Dialog::Rename(e));
                    },
                    on_delete: move |e: DriveEntry| {
                        dialog_error.set(None);
                        dialog.set(Dialog::Delete(e));
                    },
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

        // ── Visionneuse images / vidéos ──
        if let Some(index) = selected() {
            Lightbox {
                photos: media_paths.clone(),
                index,
                on_close: move |_| selected.set(None),
                on_navigate: move |new_index| selected.set(Some(new_index)),
            }
        }

        // ── Boîte de dialogue (nouveau dossier / renommer / supprimer) ──
        if dialog_open {
            div {
                class: "fixed inset-0 z-40 bg-black/50 flex items-center justify-center p-4",
                onclick: move |_| dialog.set(Dialog::None),
                div {
                    class: "bg-white rounded-2xl shadow-2xl p-6 w-full max-w-sm",
                    onclick: move |evt| evt.stop_propagation(),
                    h3 { class: "text-lg font-bold text-gray-800 mb-3", "{dialog_title}" }

                    if needs_input {
                        input {
                            class: "w-full px-3 py-2 border border-gray-200 rounded-xl focus:outline-none focus:border-rose-400",
                            r#type: "text",
                            autofocus: true,
                            placeholder: "{input_placeholder}",
                            value: "{dialog_input}",
                            oninput: move |evt| dialog_input.set(evt.value()),
                            onkeydown: move |evt| {
                                if evt.key() == Key::Enter {
                                    submit_dialog();
                                }
                            },
                        }
                    } else {
                        p { class: "text-sm text-gray-600 break-words", "{dialog_text}" }
                    }

                    if let Some(err) = dialog_error() {
                        p { class: "text-sm text-rose-600 mt-3", "{err}" }
                    }

                    div { class: "mt-5 flex justify-end gap-2",
                        button {
                            class: "px-4 py-2 rounded-xl bg-gray-100 text-gray-700 font-semibold hover:bg-gray-200 transition-colors",
                            onclick: move |_| dialog.set(Dialog::None),
                            "{cancel_label}"
                        }
                        button {
                            class: "{confirm_class}",
                            onclick: move |_| submit_dialog(),
                            "{confirm_label}"
                        }
                    }
                }
            }
        }
    }
}

/// Une tuile du explorateur : dossier ou fichier, avec ses actions.
#[component]
fn EntryTile(
    entry: DriveEntry,
    dir: String,
    media_index: Option<usize>,
    on_open_folder: EventHandler<String>,
    on_open_media: EventHandler<usize>,
    on_rename: EventHandler<DriveEntry>,
    on_delete: EventHandler<DriveEntry>,
) -> Element {
    let path = join_path(&dir, &entry.name);
    let url = file_url(&path);
    let kind = entry.kind();
    let display = entry.display_name();
    let size_label = if entry.is_dir { String::new() } else { human_size(entry.size) };

    let folder_path = path.clone();
    let rename_entry_clone = entry.clone();
    let delete_entry_clone = entry.clone();

    let open_label = t("open");
    let download_label = t("download");
    let rename_label = t("rename");
    let delete_label = t("delete");

    rsx! {
        div { class: "flex flex-col rounded-xl bg-white border border-gray-100 shadow-sm overflow-hidden",

            // ── Aperçu ──
            match kind {
                FileKind::Folder => rsx! {
                    div {
                        class: "aspect-square flex items-center justify-center bg-amber-50 hover:bg-amber-100 text-6xl cursor-pointer transition-colors",
                        title: "{open_label}",
                        onclick: move |_| on_open_folder.call(folder_path.clone()),
                        "📁"
                    }
                },
                FileKind::Image => rsx! {
                    div {
                        class: "relative aspect-square overflow-hidden bg-gray-100 cursor-pointer group",
                        onclick: move |_| on_open_media.call(media_index.unwrap_or(0)),
                        img {
                            src: "{url}",
                            loading: "lazy",
                            class: "w-full h-full object-cover transition-transform duration-200 group-hover:scale-105",
                        }
                    }
                },
                FileKind::Video => rsx! {
                    div {
                        class: "relative aspect-square overflow-hidden bg-gray-100 cursor-pointer group",
                        onclick: move |_| on_open_media.call(media_index.unwrap_or(0)),
                        video {
                            src: "{url}",
                            muted: true,
                            preload: "metadata",
                            class: "w-full h-full object-cover transition-transform duration-200 group-hover:scale-105",
                        }
                        div { class: "absolute inset-0 flex items-center justify-center pointer-events-none",
                            div { class: "w-12 h-12 rounded-full bg-black/50 backdrop-blur-sm flex items-center justify-center text-white text-xl",
                                "▶"
                            }
                        }
                    }
                },
                // Autres fichiers : ouverts dans un nouvel onglet (PDF, texte...) ou téléchargés par le navigateur
                _ => rsx! {
                    a {
                        class: "aspect-square flex items-center justify-center bg-gray-50 hover:bg-gray-100 text-5xl transition-colors",
                        href: "{url}",
                        target: "_blank",
                        rel: "noopener",
                        title: "{open_label}",
                        "{kind.icon()}"
                    }
                },
            }

            // ── Nom + actions ──
            div { class: "flex items-center gap-1 px-2 py-2",
                div { class: "flex-1 min-w-0",
                    p { class: "text-xs font-medium text-gray-700 truncate", title: "{display}", "{display}" }
                    if !size_label.is_empty() {
                        p { class: "text-[10px] text-gray-400", "{size_label}" }
                    }
                }
                if !entry.is_dir {
                    a {
                        class: "p-1.5 rounded-lg hover:bg-gray-100 text-sm",
                        href: "{url}",
                        download: "{display}",
                        title: "{download_label}",
                        "⬇️"
                    }
                }
                button {
                    class: "p-1.5 rounded-lg hover:bg-gray-100 text-sm",
                    title: "{rename_label}",
                    onclick: move |_| on_rename.call(rename_entry_clone.clone()),
                    "✏️"
                }
                button {
                    class: "p-1.5 rounded-lg hover:bg-rose-50 text-sm",
                    title: "{delete_label}",
                    onclick: move |_| on_delete.call(delete_entry_clone.clone()),
                    "🗑️"
                }
            }
        }
    }
}
