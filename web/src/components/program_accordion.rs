use dioxus::prelude::*;

// 1. Define your Program data structure
#[derive(Clone)]
struct Program {
    title: String,
    description: String,
    features: Vec<String>,
    cta_label: String,
    cta_url: String,
}

// 2. Create the accordion component
#[component]
pub fn ProgramsAccordion() -> Element {
    let expanded_index = use_signal(|| None::<usize>);

    // Sample data - replace with your actual programs
    let programs = vec![
        Program {
            title: "Business Purpose Loans".into(),
            description: "Specialized processing for investment property financing with streamlined documentation requirements.".into(),
            features: vec![
                "Quick turnaround".into(),
                "No income verification".into(),
                "Commercial properties".into(),
            ],
            cta_label: "View Details".into(),
            cta_url: "#business-purpose".into(),
        },
        Program {
            title: "Self-Employed Solutions".into(),
            description: "Custom loan processing for entrepreneurs and freelancers with non-traditional income verification.".into(),
            features: vec![
                "Bank statement programs".into(),
                "12-24 month averages".into(),
                "Flexible DTI calculations".into(),
            ],
            cta_label: "Learn More".into(),
            cta_url: "#self-employed".into(),
        },
    ];

    rsx! {
        div { class: "max-w-4xl mx-auto",
            h2 { class: "text-3xl font-bold text-center mb-8 text-gray-900", 
                "Our Loan Programs" 
            }
            
            div { class: "space-y-4",
                for (index, program) in programs.iter().enumerate() {
                    div { 
                        key: "{index}",
                        class: "border border-gray-200 rounded-xl overflow-hidden transition-all duration-200 hover:border-blue-300",
                        
                        // Accordion header
                        button { 
                            class: "w-full flex justify-between items-center p-6 text-left",
                            onclick: move |_| {
                                expanded_index.set(
                                    if expanded_index() == Some(index) {
                                        None
                                    } else {
                                        Some(index)
                                    }
                                )
                            },
                            aria_expanded: expanded_index() == Some(index),
                            
                            h3 { class: "text-xl font-semibold text-gray-900", 
                                program.title 
                            }
                            
                            // Animated chevron icon
                            svg { 
                                class: "h-6 w-6 transform transition-transform duration-200 {if expanded_index() == Some(index) { \"rotate-180\" }}",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M19 9l-7 7-7-7"
                                }
                            }
                        }
                        
                        // Accordion content
                        div { 
                            class: "px-6 pb-6 transition-all duration-300 {if expanded_index() != Some(index) { \"max-h-0 opacity-0 overflow-hidden\" } else { \"max-h-96 opacity-100\" }}",
                            
                            p { class: "text-gray-600 mb-4", 
                                program.description 
                            }
                            
                            ul { class: "list-disc list-inside space-y-2 mb-6 pl-4 text-gray-700",
                                for feature in &program.features {
                                    li { 
                                        class: "flex items-start",
                                        svg { 
                                            class: "flex-shrink-0 h-5 w-5 text-blue-500 mr-2 mt-0.5",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                                clip_rule: "evenodd"
                                            }
                                        }
                                        span { feature }
                                    }
                                }
                            }
                            
                            a { 
                                href: "{program.cta_url}",
                                class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700",
                                program.cta_label
                            }
                        }
                    }
                }
            }
        }
    }
}