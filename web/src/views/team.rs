use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Team() -> Element {
    rsx! {
        h2 { class: "font-bold text-2xl inline-block pb-1 border-b-2 border-yellow-400 hover:border-yellow-600 transition-colors",
            "Our Team"
        }
    }
}
