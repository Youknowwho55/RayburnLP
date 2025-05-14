use dioxus::prelude::*;

#[component]
pub fn BuildingIcon() -> Element {
    rsx! {

        svg {
            fill: "#000000",
            height: "200px",
            id: "Layer_1",
            "space": "preserve",
            version: "1.1",
            view_box: "0 0 512 512",
            width: "200px",
            "xlink": "http://www.w3.org/1999/xlink",
            xmlns: "http://www.w3.org/2000/svg",
            g { id: "SVGRepo_bgCarrier", stroke_width: "0" }
            g {
                id: "SVGRepo_tracerCarrier",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            g { id: "SVGRepo_iconCarrier",
                g {
                    polygon {
                        points: "176.552,97.103 26.483,187.145 26.483,459.034 326.621,459.034 326.621,187.145 ",
                        style: "fill:#D4C8A4;",
                    }
                    polygon {
                        points: "61.793,353.103 150.069,353.103 150.069,264.828 61.793,264.828 ",
                        style: "fill:#28BAE8;",
                    }
                    g {
                        polygon {
                            points: "97.103,353.103 114.759,353.103 114.759,264.828 97.103,264.828 ",
                            style: "fill:#1E94AF;",
                        }
                        polygon {
                            points: "61.793,317.793 150.069,317.793 150.069,300.138 61.793,300.138 ",
                            style: "fill:#1E94AF;",
                        }
                    }
                    polygon {
                        points: "176.552,52.966 0,158.897 0,203.034 176.552,97.103 353.103,203.034 353.103,158.897 ",
                        style: "fill:#8B8E8E;",
                    }
                    polygon {
                        points: "326.621,459.034 494.345,459.034 494.345,238.345 326.621,238.345 ",
                        style: "fill:#B5A888;",
                    }
                    polygon {
                        points: "326.621,459.034 459.034,459.034 459.034,282.483 326.621,282.483 ",
                        style: "fill:#8B8E8E;",
                    }
                    g {
                        polygon {
                            points: "326.621,361.931 459.034,361.931 459.034,344.276 326.621,344.276 ",
                            style: "fill:#707272;",
                        }
                        polygon {
                            points: "326.621,326.621 459.034,326.621 459.034,308.966 326.621,308.966 ",
                            style: "fill:#707272;",
                        }
                        polygon {
                            points: "326.621,397.241 459.034,397.241 459.034,379.586 326.621,379.586 ",
                            style: "fill:#707272;",
                        }
                        polygon {
                            points: "326.621,432.552 459.034,432.552 459.034,414.897 326.621,414.897 ",
                            style: "fill:#707272;",
                        }
                        polygon {
                            points: "353.103,185.379 353.103,203.034 326.621,187.145 326.621,238.345 512,238.345 512,185.379 ",
                            style: "fill:#707272;",
                        }
                    }
                    g {
                        polygon {
                            points: "26.483,432.552 185.379,432.552 185.379,414.897 26.483,414.897 ",
                            style: "fill:#B5A888;",
                        }
                        polygon {
                            points: "291.31,432.552 326.621,432.552 326.621,414.897 291.31,414.897 ",
                            style: "fill:#B5A888;",
                        }
                        path {
                            d: "M158.897,361.931H52.966c-4.873,0-8.828-3.955-8.828-8.828c0-4.873,3.955-8.828,8.828-8.828 h105.931c4.873,0,8.828,3.955,8.828,8.828C167.724,357.976,163.769,361.931,158.897,361.931",
                            style: "fill:#B5A888;",
                        }
                    }
                    polygon {
                        points: "185.379,459.034 291.31,459.034 291.31,264.828 185.379,264.828 ",
                        style: "fill:#C69D7D;",
                    }
                    path {
                        d: "M264.828,370.759H256c-4.873,0-8.828-3.955-8.828-8.828s3.955-8.828,8.828-8.828h8.828 c4.873,0,8.828,3.955,8.828,8.828S269.7,370.759,264.828,370.759",
                        style: "fill:#9E7D61;",
                    }
                }
            }
        }
    }
}

// Add more SVG components as needed
#[component]
pub fn AnotherIcon() -> Element {
    rsx! {
        svg {
            // Another SVG definition
        }
    }
}