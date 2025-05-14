use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TeamMemberProps {
    name: String,
    title: String,
    phone: String,
    email: String,
    nmls: String,
    #[props(default = "".to_string())]
    photo_url: String, // Default empty if no photo
}

#[component]
pub fn TeamMember(props: TeamMemberProps) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow-md overflow-hidden w-full max-w-sm mx-auto",
            // Photo section
            div { class: "h-48 bg-gray-200 flex items-center justify-center overflow-hidden",
                if props.photo_url.is_empty() {
                    svg {
                        class: "h-full w-full text-gray-400",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M24 20.993V24H0v-2.996A14.977 14.977 0 0112.004 15c4.904 0 9.26 2.354 11.996 5.993zM16.002 8.999a4 4 0 11-8 0 4 4 0 018 0z" }
                    }
                } else {
                    img {
                        class: "h-full w-full object-cover",
                        src: "{props.photo_url}",
                        alt: "Photo of {props.name}",
                    }
                }
            }
            // Info section
            div { class: "p-6",
                // Name and Title
                div { class: "mb-4",
                    h3 { class: "text-xl font-bold text-gray-900", "{props.name}" }
                    p { class: "text-gray-600", "{props.title}" }
                }
                // Contact Info
                div { class: "space-y-3",
                    div { class: "flex items-center",
                        svg {
                            class: "h-5 w-5 text-gray-500 mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z",
                            }
                        }
                        a {
                            href: "tel:{props.phone}",
                            class: "text-gray-700 hover:text-blue-600",
                            "{props.phone}"
                        }
                    }
                    div { class: "flex items-center",
                        svg {
                            class: "h-5 w-5 text-gray-500 mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
                            }
                        }
                        a {
                            href: "mailto:{props.email}",
                            class: "text-gray-700 hover:text-blue-600",
                            "{props.email}"
                        }
                    }
                    div { class: "flex items-center",
                        svg {
                            class: "h-5 w-5 text-gray-500 mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
                            }
                        }
                        span { class: "text-gray-700", "NMLS # {props.nmls}" }
                    }
                }
            }
        }
    }
}