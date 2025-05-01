use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme,ButtonSize};

#[component]
pub fn About() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };

    rsx! {

        h2 { class:"", "What do we do?" }
        p { class:"", "At Rayburn Loan Processing, we are a Third Part Processing Company Paid Directly by the Buyer.
        We specialize in providing comprehensive loan processing services to help borrowers and Brokers achieve a seamless and efficient experience by managing every aspect of the loan application process, 
        ensuring that your path from New Lead to Funding is smooth, fast, and stress-free." }

        div {  class:"flex flex-col gap-4",
            h2 { class:"", "Our Services" }
            p { class:"", "We offer a range of services to meet the needs of our clients, including:" }
            ul { class:"list-disc list-inside",
                li { "Reviewing Borrower Documents" }
                li { "Opening Title and Escrow" }
                li { "Coordinating Appraisals" }
                li { "Correspondence with Underwriters" }
                li { "Final Closing Coordination" }
                li { "Compliance and Regulatory Review" }
                li { "Loan Registration and Setup" }
            }
        }


        Button {
            button_scheme: ButtonScheme::Success,
            button_size: ButtonSize::Large,
            on_click: handle_click,
            "Success Button"
        }
    }
}
