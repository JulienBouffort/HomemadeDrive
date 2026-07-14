use dioxus::prelude::*;


#[component]
pub fn Status(
    photos: Signal<Vec<crate::models::upload_state::PhotoFile>>
) -> Element {


    let count = photos.read().len();


    rsx! {

        p { class: "text-center mt-6 text-gray-500",

            if count == 0 {
                "Aucune photo sélectionnée"
            } else {
                "{count} photo(s) prête(s) à envoyer"
            }
        
        }
    }
}