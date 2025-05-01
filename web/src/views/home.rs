use dioxus::prelude::*;
use crate::views::{About, Pricing, Programs, Hero};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // let handle_click = move |_| {
    //     println!("Button clicked!");
    // };
    rsx! {

       Hero{}
       Programs{}
       About {}
       Pricing {} 
    }
}
