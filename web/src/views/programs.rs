use dioxus::prelude::*;
use crate::components::button::{Button, ButtonScheme, ButtonSize};
use crate::components::card::{Card, CardTitle, CardBody};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Programs() -> Element {
    rsx! {
        Card {
            // href: "https://example.com",
            // image_src: Some("https://example.com/image.jpg"),
            // image_alt: Some("Card header image"),
            CardTitle {
                "Business Purpose Real Estate"
            }
            CardBody {
                "Card Body"
            }
        }

            Card {
                // href: "https://example.com",
                // image_src: Some("https://example.com/image.jpg"),
                // image_alt: Some("Card header image"),
                CardTitle {
                    "Self Employed"
                }
                CardBody {
                    "Card Body"
                }
        }

        Card {
            // href: "https://example.com",
            // image_src: Some("https://example.com/image.jpg"),
            // image_alt: Some("Card header image"),
            CardTitle {
                "Business Purpose Real Estate"
            }
            CardBody {
                "Card Body"
            }
    }

    Card {
        // href: "https://example.com",
        // image_src: Some("https://example.com/image.jpg"),
        // image_alt: Some("Card header image"),
        CardTitle {
            "Business Purpose Real Estate"
        }
        CardBody {
            "Card Body"
        }
    }
}
}



// Button {
//     button_scheme: ButtonScheme::Custom,
//     button_size: ButtonSize::Large,
//     on_click: handle_click,
//     "Learn More"
// }  




