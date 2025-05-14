use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme, ButtonSize};
use crate::components::BuildingIcon; // Assuming you have this SVG component

#[component]
pub fn About() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };

    rsx! {
        div { class: "space-y-8", // Adds vertical spacing between sections
            // Header section with icon
            div { class: "flex flex-col md:flex-row items-start gap-8",
                // Text content (left side)
                div { class: "md:w-2/3 space-y-4",
                    h2 { class: "font-bold text-2xl inline-block pb-1 border-b-2 border-yellow-400 hover:border-yellow-600 transition-colors",
                        "What do we do?"
                    }
                    p { class: "text-gray-700 leading-relaxed",
                        "At Rayburn Loan Processing, we are a Third Party Processing Company Paid Directly by the Buyer.
                        We specialize in providing comprehensive loan processing services to help borrowers and Brokers achieve a seamless and efficient experience by managing every aspect of the loan application process, 
                        ensuring that your path from New Lead to Funding is smooth, fast, and stress-free."
                    }
                }
                // SVG icon (right side)
                div { class: "md:w-1/3 flex justify-center md:justify-end", BuildingIcon {} }
            }

            // Services section
            div { class: "flex flex-col gap-4 bg-gray-50 p-6 rounded-lg",
                h2 { class: "font-bold text-xl text-gray-800", "Our Services" }
                p { class: "text-gray-700",
                    "We offer a range of services to meet the needs of our clients, including:"
                }
                ul { class: "list-disc list-inside space-y-2 text-gray-700",
                    li { "Reviewing Borrower Documents" }
                    li { "Opening Title and Escrow" }
                    li { "Coordinating Appraisals" }
                    li { "Correspondence with Underwriters" }
                    li { "Final Closing Coordination" }
                    li { "Compliance and Regulatory Review" }
                    li { "Loan Registration and Setup" }
                }
            }

            // Button centered below
            div { class: "flex justify-center",
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