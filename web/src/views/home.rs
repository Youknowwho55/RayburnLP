use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme,ButtonSize};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
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
