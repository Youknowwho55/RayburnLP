use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme,ButtonSize};

#[component]
pub fn About() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };

    rsx! {

        p { class:"", "This is the About section" }
        Button {
            button_scheme: ButtonScheme::Success,
            button_size: ButtonSize::Large,
            on_click: handle_click,
            "Success Button"
        }
    }
}
