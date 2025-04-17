use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Pricing() -> Element {
    rsx! {
        p { "This is the pricing section" }
    }
}
