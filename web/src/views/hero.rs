use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme, ButtonSize};
use crate::components::icons::BuildingIcon;
// const LOGO: Asset = asset!("/assets/logo.png");


#[component]
pub fn Hero() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };

    rsx! {
        section { class: "bg-gradient-to-br from-yellow-50 to-yellow-100 py-6 flex items-center justify-center p-4",
            div { class: "container mx-auto flex flex-col md:flex-row items-center justify-between gap-8",
                // SVG on left (top on mobile)
                div { class: "md:w-1/2 flex justify-center", BuildingIcon {} }

                // Content on right (bottom on mobile)
                div { class: "md:w-1/2 flex flex-col items-center md:items-start text-center md:text-left",
                    h1 { class: "text-3xl md:text-4xl font-bold text-yellow-600 mb-6",
                        "Streamlined Loan Processing - Fee Covered by the Borrower at Closing!"
                    }
                    div { class: "mt-6",
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
    }
}