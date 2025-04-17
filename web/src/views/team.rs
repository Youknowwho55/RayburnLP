use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Team() -> Element {
    rsx! {
        p { "This is the Teams section" }
    }
}
