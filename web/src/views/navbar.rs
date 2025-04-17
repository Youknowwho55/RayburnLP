use crate::Route;
use dioxus::prelude::*;


/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        nav { id: "navbar", class: "w-full bg-gray-900 text-white shadow-md",
            div { class: "max-w-6xl mx-auto px-4 py-2 flex items-center justify-between",
                h1 { class: "text-xl font-semibold", "MyApp" }
                div { class: "flex gap-6 items-center text-sm",
                    Link {
                        to: Route::Home {},
                        class: "hover:text-blue-400 transition",
                        "Home"
                    }

                    Link {
                        to: Route::About {},
                        class: "hover:text-blue-400 transition",
                        "About Us"
                    }
                    Link {
                        to: Route::Team { },
                        class: "hover:text-blue-400 transition",
                        "Team"
                    }
                    Link {
                        to: Route::Pricing {  },
                        class: "hover:text-blue-400 transition",
                        "Pricing"
                    }
                    Link {
                        to: Route::Resources {  },
                        class: "hover:text-blue-400 transition",
                        "Resources"
                    }
                    Link {
                        to: Route::Contact {  },
                        class: "hover:text-blue-400 transition",
                        "Contact Us"
                    }


                }
            }
        }
        // Outlet::<Route> {}


}
}



