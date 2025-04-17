use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        p { class:"w-4 h-4 text-yellow-300 ms-1", "This is the About section" }
    }
}
