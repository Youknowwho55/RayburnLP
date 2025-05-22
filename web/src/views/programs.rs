use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme, ButtonSize};
use crate::components::card::{Card, CardTitle, CardBody};
use crate::components::{ArrowRight, HomeCheck, HomeGarage, HomeHeart, HomePerson};

#[component]
pub fn Programs() -> Element {

    rsx! {
        div { class: " font-semibold gap-4 p-4 ",
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
                        ArrowRight {  }
                    }
                }

                // Card 2
                div { class: "md:flex-1",
                    Card {
                        HomePerson {}
                        CardTitle { "Self Employed" }
                        CardBody { "Card Body" }
                        ArrowRight {  }

                    }
                }

                // Card 3
                div { class: "md:flex-1",
                    Card {
                        HomeCheck {}
                        CardTitle { "Conventional" }
                        CardBody { "Card Body" }
                        ArrowRight {}

                    }
                }

                // Card 4
                div { class: "md:flex-1",
                    Card {
                        HomeHeart {}
                        CardTitle { "FHA and VA" }
                        CardBody { "Card Body" }
                        ArrowRight {}
                    }
                }
            }
        }
    }
}



