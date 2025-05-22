use dioxus::prelude::*;
use crate::components::ResourceTable;
use crate::components::ResourceItem;

#[component]
pub fn Resources() -> Element {
        let resources = vec![
        ResourceItem {
            name: "Pre-Approval Checklist".into(),
            file_type: "PDF".into(),
            size: "2.4 MB".into(),
            last_updated: "2023-11-15".into(),
            download_url: "/downloads/pre-approval.pdf".into(),
        },
        ResourceItem {
            name: "Loan Application Form".into(),
            file_type: "DOCX".into(),
            size: "1.2 MB".into(),
            last_updated: "2023-10-28".into(),
            download_url: "/downloads/application.docx".into(),
        },
    ];
    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
            // Header section
            div { class: "text-center mb-16",
                h2 { class: "text-4xl font-extrabold text-gray-900 sm:text-5xl mb-6",
                    "Resources"
                }
                p { class: "max-w-3xl mx-auto text-xl text-gray-600",
                    "Essential tools and documents to streamline your loan processing workflow"
                }
            }

            // Resources grid
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                // Resource Card 1
                div { class: "bg-white rounded-xl shadow-md overflow-hidden hover:shadow-lg transition-shadow duration-300",
                    div { class: "p-8",
                        div { class: "flex items-center mb-4",
                            div { class: "flex-shrink-0 bg-blue-100 p-3 rounded-lg",
                                svg {
                                    class: "h-8 w-8 text-blue-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2",
                                    }
                                }
                            }
                            h3 { class: "ml-4 text-xl font-semibold text-gray-900",
                                "Pre-Approval Document Checklist"
                            }
                        }
                        p { class: "text-gray-600",
                            "A downloadable PDF that outlines all necessary documents to begin the pre-approval process, reducing delays and speeding up approvals."
                        }
                        div { class: "mt-6",
                            a {
                                href: "#",
                                class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700",
                                "Download PDF"
                            }
                        }
                    }
                }

                // Resource Card 2
                div { class: "bg-white rounded-xl shadow-md overflow-hidden hover:shadow-lg transition-shadow duration-300",
                    div { class: "p-8",
                        div { class: "flex items-center mb-4",
                            div { class: "flex-shrink-0 bg-green-100 p-3 rounded-lg",
                                svg {
                                    class: "h-8 w-8 text-green-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
                                    }
                                }
                            }
                            h3 { class: "ml-4 text-xl font-semibold text-gray-900",
                                "Loan Approval Roadmap"
                            }
                        }
                        p { class: "text-gray-600",
                            "A visual flowchart PDF outlining key steps from application to final approval, perfect for explaining the process to clients."
                        }
                        div { class: "mt-6",
                            a {
                                href: "#",
                                class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-green-600 hover:bg-green-700",
                                "Download PDF"
                            }
                        }
                    }
                }

                // Resource Card 3
                div { class: "bg-white rounded-xl shadow-md overflow-hidden hover:shadow-lg transition-shadow duration-300",
                    div { class: "p-8",
                        div { class: "flex items-center mb-4",
                            div { class: "flex-shrink-0 bg-purple-100 p-3 rounded-lg",
                                svg {
                                    class: "h-8 w-8 text-purple-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
                                    }
                                }
                            }
                            h3 { class: "ml-4 text-xl font-semibold text-gray-900",
                                "Loan Document Templates"
                            }
                        }
                        ul { class: "list-disc list-inside space-y-2 text-gray-600 mb-4",
                            li { "Sample 1003" }
                            li { "Borrower Authorization" }
                            li { "Credit Card Authorization" }
                            li { "Gift Funds Letter" }
                            li { "DTI Calculation Worksheet" }
                            li { "Property Appraisal Request" }
                        }
                        div { class: "mt-4",
                            a {
                                href: "#",
                                class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-purple-600 hover:bg-purple-700",
                                "Download Templates"
                            }
                        }
                    }
                }
            }
        }
        div { class: "px-4 sm:px-6 lg:px-8 py-8",
            h2 { class: "text-2xl font-bold mb-6", "Resource Library" }
            ResourceTable { resources: resources.clone() }
        }
    }
}





