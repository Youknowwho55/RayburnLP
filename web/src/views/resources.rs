use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Resources() -> Element {
    rsx! {
        p { "This is the Resources section" }
    }
}
