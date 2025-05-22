use dioxus::prelude::*;
use crate::components::card::{Card, CardTitle, CardBody};
use crate::components::{ArrowRight, HomeCheck, HomeGarage, HomeHeart, HomePerson};

#[component]
pub fn Programs() -> Element {

    rsx! {
        div { class: " font-semibold  p-4 ",
            // Header with centered H3 and orange underline
            div { class: "flex justify-center mb-6",
                h3 { class: "text-2xl font-bold text-center border-b-2 border-orange-500 pb-2",
                    "Programs"
                }
            }
            // Cards container
            div { class: "flex flex-col md:flex-row  p-2",
                // Card 1
                div { class: "md:flex-1 p-2",
                    Card {
                        HomeGarage {}
                        CardTitle { "Business Purpose Real Estate" }
CardBody {
  "Flexible loans tailored for real estate investors and business owners — no personal income verification required."
}
                        ArrowRight {  }
                    }
                }

                // Card 2
                div { class: "md:flex-1 p-2",
                    Card {
                        HomePerson {}
                        CardTitle { "Self Employed" }
                        CardBody {
                            "Loans designed for self-employed individuals with unique income situations."
                        }
                        ArrowRight {  }

                    }
                }

                // Card 3
                div { class: "md:flex-1 p-2",
                    Card {
                        HomeCheck {}
                        CardTitle { "Conventional" }
                        CardBody {
                            "Conventional loans with competitive rates and flexible terms."
                        }
                        ArrowRight {}

                    }
                }

                // Card 4
                div { class: "md:flex-1 p-2",
                    Card {
                        HomeHeart {}
                        CardTitle { "FHA and VA" }
                        CardBody {
                            "FHA and VA loans with flexible guidelines and low down payment options."
                        }
                        ArrowRight {}
                    }
                }
            }
        }
    }
}



