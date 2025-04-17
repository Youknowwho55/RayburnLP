use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme, ButtonSize};
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
// web\src\components\hero.rs
// web\components\button.rs
#[component]
pub fn Hero() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };
    rsx! {

        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            // img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                // The RSX macro also supports text nodes surrounded by quotes
            p{class:"", "Streamlined Loan Processing - Fee Covered by the Borrower at Closing!"
            Button {
                button_scheme: ButtonScheme::Success,
                button_size: ButtonSize::Large,
                on_click: handle_click,
                "Success Button"
            }

        }
            }
        }
    }
}
