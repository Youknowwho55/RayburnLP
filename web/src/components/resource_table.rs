use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ResourceItem {
    pub name: String,
    pub file_type: String,
    pub size: String,
    pub last_updated: String,
    #[props(default = "#".to_string())]
    pub download_url: String,
}

#[component]
pub fn ResourceTable(resources: Vec<ResourceItem>) -> Element {
    rsx! {
        div { class: "overflow-hidden shadow ring-1 ring-black ring-opacity-5 rounded-lg",
            table { class: "min-w-full divide-y divide-gray-300",
                thead { class: "bg-gray-50",
                    tr {
                        th {
                            class: "px-4 py-3.5 text-left text-sm font-semibold text-gray-900",
                            scope: "col",
                            "Document"
                        }
                        th {
                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                            scope: "col",
                            "Type"
                        }
                        th {
                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                            scope: "col",
                            "Size"
                        }
                        th {
                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                            scope: "col",
                            "Updated"
                        }
                        th {
                            class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900",
                            scope: "col",
                            span { class: "sr-only", "Download" }
                        }
                    }
                }
                tbody { class: "divide-y divide-gray-200 bg-white",
                    for resource in resources.iter() {
                        tr { key: "{resource.name}",
                            td { class: "whitespace-nowrap px-4 py-4 text-sm font-medium text-gray-900",
                                "{resource.name}"
                            }
                            td { class: "whitespace-nowrap px-3 py-4 text-sm text-gray-500",
                                "{resource.file_type}"
                            }
                            td { class: "whitespace-nowrap px-3 py-4 text-sm text-gray-500",
                                "{resource.size}"
                            }
                            td { class: "whitespace-nowrap px-3 py-4 text-sm text-gray-500",
                                "{resource.last_updated}"
                            }
                            td { class: "whitespace-nowrap px-3 py-4 text-sm font-medium",
                                a {
                                    href: "{resource.download_url}",
                                    class: "text-blue-600 hover:text-blue-900",
                                    "Download"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}