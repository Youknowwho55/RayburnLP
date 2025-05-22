use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme, ButtonSize};
use crate::components::card::{Card, CardTitle, CardBody};
use crate::components::HomeGarage;

#[component]
pub fn Programs() -> Element {

    rsx! {
        div { class: "text-white font-semibold gap-4 p-4 bg-lpBlue",
            // Header with centered H3 and orange underline
            div { class: "flex justify-center mb-6",
                h3 { class: "text-2xl font-bold text-center border-b-2 border-orange-500 pb-2",
                    "Programs"
                }
            }
            // Cards container
            div { class: "flex flex-col md:flex-row gap-4 p-4",
                // Card 1
                div { class: "md:flex-1 ",
                    Card {
                        HomeGarage {}
                        CardTitle { "Business Purpose Real Estate" }
                        CardBody { "Card Body" }
                    }
                }

                // Card 2
                div { class: "md:flex-1",
                    Card {
                        CardTitle { "Self Employed" }
                        CardBody { "Card Body" }
                    }
                }

                // Card 3
                div { class: "md:flex-1",
                    Card {
                        CardTitle { "Business Purpose Real Estate" }
                        CardBody { "Card Body" }
                    }
                }

                // Card 4
                div { class: "md:flex-1",
                    Card {
                        CardTitle { "Business Purpose Real Estate" }
                        CardBody { "Card Body" }
                    }
                }
            }
        }
    }
}



