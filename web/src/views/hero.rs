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

        section { class: "flex items-center justify-center bg-gradient-to-br from-yellow-50 to-yellow-100",
            div { class: "text-center p-8 max-w-4xl",
                h1 { class: "text-4xl md:text-4xl font-bold text-yellow-600 mb-4", "Streamlined Loan Processing - Fee Covered by the Borrower at Closing!" }
                Button {
                    button_scheme: ButtonScheme::Custom,
                    button_size: ButtonSize::Large,
                    on_click: handle_click,
                    "Get in touch with us!"
                }            
            }

        }
    }
}
