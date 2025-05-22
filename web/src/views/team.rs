use dioxus::prelude::*;
use crate::components::team_member::TeamMember;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Team() -> Element {
    rsx! {

            div { class: "flex justify-center mb-6",


        h3 { class: "font-bold text-2xl inline-block pb-1 border-b-2 border-yellow-400 hover:border-yellow-600 transition-colors",
            "Our Team"
        }
    }
        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 p-8",
            TeamMember {
                name: "John Doe".to_string(),
                title: "Loan Processor".to_string(),
                phone: "(555) 123-4567".to_string(),
                email: "john@example.com".to_string(),
                nmls: "123456".to_string(),
                photo_url: "/path/to/photo.jpg".to_string(),
            }
            TeamMember {
                name: "John Doe".to_string(),
                title: "Loan Processor".to_string(),
                phone: "(555) 123-4567".to_string(),
                email: "john@example.com".to_string(),
                nmls: "123456".to_string(),
                photo_url: "/path/to/photo.jpg".to_string(),
            }
                // Add more team members...
        }
    }
}



