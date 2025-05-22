use dioxus::prelude::*;
use crate::components::card::{Card, CardTitle, CardBody};


#[component]
pub fn Pricing() -> Element {
    rsx! {
        div { class: " font-semibold gap-4 p-4",
            // Header with centered H3 and orange underline
            div { class: "flex justify-center mb-6",
                h3 { class: "text-2xl font-bold text-center border-b-2 border-orange-500 pb-2",
                    "Pricing"
                }
            }
            // Cards container
            div { class: "flex flex-col md:flex-row gap-4 p-4",
                // Card 1
                div { class: "md:flex-1",
                    Card { class: "relative", // Added for positioning
                        CardTitle { "Standard Fee" }
                        CardBody { class: "pr-20", // Add right padding to prevent text overlap
                            "Non-QM"
                            br {}
                            "Business Purpose"
                            br {}
                            "Conventional"
                            br {}
                            "Hard Money"
                        }
                        // Circle positioned on right
                        div { class: "absolute top-8 right-8 flex items-center justify-center w-20 h-20 rounded-full bg-blue-600 text-white font-bold text-xl",
                            "$1200"
                        }
                    }
                }

                // Card 2
                div { class: "md:flex-1",
                    Card { class: "relative", // Added for positioning
                        // href: "https://example.com",
                        // image_src: Some("https://example.com/image.jpg"),
                        // image_alt: Some("Card header image"),
                        CardTitle { "Other" }
                        CardBody {
                            "IRRRL"
                            br {}
                            "FHA Streamline"
                            br {}
                            "Seconds"
                            br {}
                            "HELOC's"
                        }
                        // Circle positioned on right
                        div { class: "absolute top-8 right-8 flex items-center justify-center w-20 h-20 rounded-full bg-blue-600 text-white font-bold text-xl",
                            "$695"
                        }
                    }
                }

                // Card 3
                div { class: "md:flex-1",
                    Card { class: "relative", // Added for positioning
                        // href: "https://example.com",
                        // image_src: Some("https://example.com/image.jpg"),
                        // image_alt: Some("Card header image"),
                        CardTitle { "Add-ons" }
                        CardBody {
                            "Re-submissions"
                            br {}
                            "Rush Requests"
                            br {}
                            "Registration"
                            br {}
                            "(Each additional service is priced individually)"
                        }
                        // Circle positioned on right
                        div { class: "absolute top-8 right-8 flex items-center justify-center w-20 h-20 rounded-full bg-blue-600 text-white font-bold text-xl",
                            "$100"
                        }
                    }
                }
            }
        }
    }
}