// src/pages/documentation.rs
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ComponentDoc {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub usage: String,
    pub props: Vec<PropDoc>,
    pub examples: Vec<ExampleDoc>,
}

#[derive(Clone, PartialEq)]
pub struct PropDoc {
    pub name: String,
    pub prop_type: String,
    pub required: bool,
    pub default: Option<String>,
    pub description: String,
}

#[derive(Clone, PartialEq)]
pub struct ExampleDoc {
    pub title: String,
    pub code: String,
    pub description: String,
}

// Temporary in-memory documentation data (later this will come from JSON)
pub fn get_component_doc(doc_id: &str) -> Option<ComponentDoc> {
    let docs = get_all_docs();
    docs.into_iter().find(|doc| doc.id == doc_id)
}

pub fn get_all_docs() -> Vec<ComponentDoc> {
    vec![
        ComponentDoc {
            id: "gallery-cell".to_string(),
            name: "GalleryCell".to_string(),
            description: "A flexible cell component for displaying content in a gallery grid with badge support and filtering capabilities.".to_string(),
            category: "Gallery".to_string(),
            usage: "Use GalleryCell inside a GalleryGridRow to create grid-based layouts with filterable content.".to_string(),
            props: vec![
                PropDoc {
                    name: "badges".to_string(),
                    prop_type: "Vec<(String, String)>".to_string(),
                    required: false,
                    default: Some("vec![]".to_string()),
                    description: "List of badges as (icon_class, label) tuples to display on the cell".to_string(),
                },
                PropDoc {
                    name: "class".to_string(),
                    prop_type: "String".to_string(),
                    required: false,
                    default: Some("String::new()".to_string()),
                    description: "Additional CSS classes to apply to the cell".to_string(),
                },
                PropDoc {
                    name: "doc_id".to_string(),
                    prop_type: "String".to_string(),
                    required: false,
                    default: Some("String::new()".to_string()),
                    description: "Documentation ID for linking to component docs".to_string(),
                },
            ],
            examples: vec![
                ExampleDoc {
                    title: "Basic Cell with Image".to_string(),
                    code: r#"GalleryCell {
    badges: vec![("mdi mdi-star".into(), "Featured".into())],
    img { src: "image.jpg", alt: "Example" }
}"#.to_string(),
                    description: "A simple cell displaying an image with a featured badge".to_string(),
                },
                ExampleDoc {
                    title: "Cell with Button".to_string(),
                    code: r#"GalleryCell {
    badges: vec![("mdi mdi-heart".into(), "Favorite".into())],
    button { "Click Me" }
}"#.to_string(),
                    description: "A cell containing an interactive button element".to_string(),
                },
            ],
        },
        ComponentDoc {
            id: "gallery-grid-row".to_string(),
            name: "GalleryGridRow".to_string(),
            description: "A responsive row container that arranges GalleryCell components in a flexible grid layout with a labeled connect panel.".to_string(),
            category: "Gallery".to_string(),
            usage: "Use GalleryGridRow inside GalleryBox to organize cells into categorized rows with automatic wrapping and filtering.".to_string(),
            props: vec![
                PropDoc {
                    name: "min_cell_width".to_string(),
                    prop_type: "usize".to_string(),
                    required: false,
                    default: Some("200".to_string()),
                    description: "Minimum width in pixels for each cell before wrapping".to_string(),
                },
                PropDoc {
                    name: "icon".to_string(),
                    prop_type: "String".to_string(),
                    required: false,
                    default: Some("\"mdi mdi-link-variant\"".to_string()),
                    description: "Icon class for the connect panel".to_string(),
                },
                PropDoc {
                    name: "label".to_string(),
                    prop_type: "String".to_string(),
                    required: false,
                    default: Some("\"Connect\"".to_string()),
                    description: "Text label for the row's connect panel".to_string(),
                },
                PropDoc {
                    name: "row_badges".to_string(),
                    prop_type: "Vec<String>".to_string(),
                    required: false,
                    default: Some("vec![]".to_string()),
                    description: "List of badge icons this row should be visible for when filtering".to_string(),
                },
            ],
            examples: vec![
                ExampleDoc {
                    title: "Basic Row".to_string(),
                    code: r#"GalleryGridRow {
    min_cell_width: 200,
    icon: "mdi mdi-image",
    label: "Images",
    
    GalleryCell { /* ... */ }
    GalleryCell { /* ... */ }
}"#.to_string(),
                    description: "A row of image cells with a 200px minimum width".to_string(),
                },
            ],
        },
        ComponentDoc {
            id: "gallery-box".to_string(),
            name: "GalleryBox".to_string(),
            description: "A container component that provides filtering, scrolling, and organization for gallery content with badge-based filtering.".to_string(),
            category: "Gallery".to_string(),
            usage: "Wrap GalleryGridRow components in GalleryBox to create a complete filterable gallery interface.".to_string(),
            props: vec![
                PropDoc {
                    name: "height".to_string(),
                    prop_type: "String".to_string(),
                    required: false,
                    default: Some("\"600px\"".to_string()),
                    description: "Height of the scrollable content area".to_string(),
                },
                PropDoc {
                    name: "available_badges".to_string(),
                    prop_type: "Vec<(String, String)>".to_string(),
                    required: false,
                    default: Some("vec![]".to_string()),
                    description: "Available badge filters as (icon_class, label) tuples".to_string(),
                },
            ],
            examples: vec![
                ExampleDoc {
                    title: "Complete Gallery".to_string(),
                    code: r#"GalleryBox {
    height: "100%",
    available_badges: vec![
        ("mdi mdi-star".into(), "Featured".into()),
        ("mdi mdi-heart".into(), "Favorite".into()),
    ],
    
    GalleryGridRow { /* ... */ }
    GalleryGridRow { /* ... */ }
}"#.to_string(),
                    description: "A full gallery with filtering capabilities".to_string(),
                },
            ],
        },
    ]
}

#[derive(Props, Clone, PartialEq)]
pub struct DocumentationPageProps {
    #[props(default = String::new())]
    pub doc_id: String,
}

#[component]
pub fn DocumentationPage(props: DocumentationPageProps) -> Element {
    let doc = if props.doc_id.is_empty() {
        None
    } else {
        get_component_doc(&props.doc_id)
    };

    rsx! {
        div {
            style: "
                padding: 32px;
                color: white;
                max-width: 1200px;
                margin: 0 auto;
                height: 100%;
                overflow-y: auto;
            ",

            if let Some(component_doc) = doc {
                // Component Documentation
                div {
                    // Header
                    div {
                        style: "margin-bottom: 32px;",
                        h1 {
                            style: "
                                font-size: 42px;
                                margin-bottom: 8px;
                                display: flex;
                                align-items: center;
                                gap: 16px;
                            ",
                            i { class: "mdi mdi-book-open-variant", style: "font-size: 48px; color: #4CAF50;" }
                            "{component_doc.name}"
                        }
                        div {
                            style: "
                                display: inline-block;
                                padding: 6px 12px;
                                background: rgba(76, 175, 80, 0.2);
                                border-radius: 6px;
                                font-size: 14px;
                                margin-bottom: 16px;
                            ",
                            "{component_doc.category}"
                        }
                        p {
                            style: "
                                font-size: 18px;
                                line-height: 1.6;
                                opacity: 0.9;
                            ",
                            "{component_doc.description}"
                        }
                    }

                    // Usage
                    div {
                        style: "
                            background: rgba(255, 255, 255, 0.08);
                            backdrop-filter: blur(10px);
                            border-radius: 12px;
                            padding: 24px;
                            margin-bottom: 24px;
                        ",
                        h2 {
                            style: "
                                font-size: 24px;
                                margin-bottom: 12px;
                                display: flex;
                                align-items: center;
                                gap: 10px;
                            ",
                            i { class: "mdi mdi-lightbulb-on", style: "color: #FFC107;" }
                            "Usage"
                        }
                        p { style: "line-height: 1.6; opacity: 0.9;", "{component_doc.usage}" }
                    }

                    // Props
                    if !component_doc.props.is_empty() {
                        div {
                            style: "
                                background: rgba(255, 255, 255, 0.08);
                                backdrop-filter: blur(10px);
                                border-radius: 12px;
                                padding: 24px;
                                margin-bottom: 24px;
                            ",
                            h2 {
                                style: "
                                    font-size: 24px;
                                    margin-bottom: 16px;
                                    display: flex;
                                    align-items: center;
                                    gap: 10px;
                                ",
                                i { class: "mdi mdi-cog", style: "color: #2196F3;" }
                                "Properties"
                            }
                            div { style: "display: flex; flex-direction: column; gap: 16px;",
                                for prop in component_doc.props.iter() {
                                    div {
                                        style: "
                                            padding: 16px;
                                            background: rgba(255, 255, 255, 0.05);
                                            border-radius: 8px;
                                            border-left: 3px solid #4CAF50;
                                        ",
                                        div {
                                            style: "display: flex; align-items: center; gap: 12px; margin-bottom: 8px;",
                                            code {
                                                style: "
                                                    font-size: 16px;
                                                    font-weight: 600;
                                                    color: #4CAF50;
                                                    font-family: 'Courier New', monospace;
                                                ",
                                                "{prop.name}"
                                            }
                                            span {
                                                style: "
                                                    font-size: 14px;
                                                    padding: 2px 8px;
                                                    background: rgba(33, 150, 243, 0.3);
                                                    border-radius: 4px;
                                                    font-family: 'Courier New', monospace;
                                                ",
                                                "{prop.prop_type}"
                                            }
                                            if prop.required {
                                                span {
                                                    style: "
                                                        font-size: 12px;
                                                        padding: 2px 6px;
                                                        background: rgba(244, 67, 54, 0.3);
                                                        border-radius: 4px;
                                                    ",
                                                    "required"
                                                }
                                            }
                                        }
                                        p { style: "opacity: 0.9; margin-bottom: 8px;", "{prop.description}" }
                                        if let Some(default_val) = &prop.default {
                                            div {
                                                style: "font-size: 14px; opacity: 0.7;",
                                                strong { "Default: " }
                                                code {
                                                    style: "
                                                        font-family: 'Courier New', monospace;
                                                        background: rgba(0, 0, 0, 0.3);
                                                        padding: 2px 6px;
                                                        border-radius: 3px;
                                                    ",
                                                    "{default_val}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Examples
                    if !component_doc.examples.is_empty() {
                        div {
                            style: "
                                background: rgba(255, 255, 255, 0.08);
                                backdrop-filter: blur(10px);
                                border-radius: 12px;
                                padding: 24px;
                            ",
                            h2 {
                                style: "
                                    font-size: 24px;
                                    margin-bottom: 16px;
                                    display: flex;
                                    align-items: center;
                                    gap: 10px;
                                ",
                                i { class: "mdi mdi-code-tags", style: "color: #FF9800;" }
                                "Examples"
                            }
                            div { style: "display: flex; flex-direction: column; gap: 20px;",
                                for example in component_doc.examples.iter() {
                                    div {
                                        h3 {
                                            style: "
                                                font-size: 18px;
                                                margin-bottom: 8px;
                                                color: #4CAF50;
                                            ",
                                            "{example.title}"
                                        }
                                        p {
                                            style: "
                                                opacity: 0.8;
                                                margin-bottom: 12px;
                                                font-size: 14px;
                                            ",
                                            "{example.description}"
                                        }
                                        pre {
                                            style: "
                                                background: rgba(0, 0, 0, 0.4);
                                                padding: 16px;
                                                border-radius: 8px;
                                                overflow-x: auto;
                                                border: 1px solid rgba(255, 255, 255, 0.1);
                                            ",
                                            code {
                                                style: "
                                                    font-family: 'Courier New', monospace;
                                                    font-size: 14px;
                                                    line-height: 1.5;
                                                    color: #e0e0e0;
                                                ",
                                                "{example.code}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // No documentation selected
                div {
                    style: "
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                        height: 100%;
                        text-align: center;
                    ",
                    i {
                        class: "mdi mdi-book-open-page-variant",
                        style: "font-size: 96px; color: rgba(255, 255, 255, 0.3); margin-bottom: 24px;"
                    }
                    h2 {
                        style: "font-size: 32px; margin-bottom: 16px;",
                        "Component Documentation"
                    }
                    p {
                        style: "font-size: 18px; opacity: 0.7; max-width: 600px;",
                        "Select a component from the gallery by clicking the documentation button on any cell, or browse the documentation menu."
                    }
                }
            }
        }
    }
}