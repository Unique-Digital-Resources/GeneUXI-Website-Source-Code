// src/pages/component_gallery.rs
use dioxus::prelude::*;
use crate::gallery::gallery_grid_row::{GalleryGridRow, GalleryCell};
use crate::gallery::gallery_box::GalleryBox;

// Sample data structure for components
#[derive(Clone, Debug, PartialEq)]
struct ComponentInfo {
    name: String,
    component_type: String,
    frameworks: Vec<String>,
}

fn get_sample_components() -> Vec<ComponentInfo> {
    vec![
        ComponentInfo {
            name: "Primary Button".to_string(),
            component_type: "Button".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        ComponentInfo {
            name: "Secondary Button".to_string(),
            component_type: "Button".to_string(),
            frameworks: vec!["React".to_string()],
        },
        ComponentInfo {
            name: "Icon Button".to_string(),
            component_type: "Button".to_string(),
            frameworks: vec!["Vue".to_string(), "Angular".to_string()],
        },
        ComponentInfo {
            name: "Card Layout".to_string(),
            component_type: "Layout".to_string(),
            frameworks: vec!["React".to_string(), "Angular".to_string()],
        },
        ComponentInfo {
            name: "Grid Layout".to_string(),
            component_type: "Layout".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        ComponentInfo {
            name: "Flex Container".to_string(),
            component_type: "Layout".to_string(),
            frameworks: vec!["Svelte".to_string()],
        },
        ComponentInfo {
            name: "Text Input".to_string(),
            component_type: "Form".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string(), "Svelte".to_string()],
        },
        ComponentInfo {
            name: "Checkbox".to_string(),
            component_type: "Form".to_string(),
            frameworks: vec!["Vue".to_string()],
        },
        ComponentInfo {
            name: "Select Dropdown".to_string(),
            component_type: "Form".to_string(),
            frameworks: vec!["React".to_string(), "Angular".to_string()],
        },
        ComponentInfo {
            name: "Data Table".to_string(),
            component_type: "Display".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        ComponentInfo {
            name: "Progress Bar".to_string(),
            component_type: "Display".to_string(),
            frameworks: vec!["React".to_string()],
        },
    ]
}

#[component]
pub fn ComponentGalleryPage() -> Element {
    let components = get_sample_components();
    
    // Available framework badges for filtering
    let framework_badges = vec![
        ("mdi mdi-react".to_string(), "React".to_string()),
        ("mdi mdi-vuejs".to_string(), "Vue".to_string()),
        ("mdi mdi-angular".to_string(), "Angular".to_string()),
        ("mdi mdi-language-javascript".to_string(), "Svelte".to_string()),
    ];
    
    // Available type badges for filtering
    let type_badges = vec![
        ("mdi mdi-gesture-tap-button".to_string(), "Button".to_string()),
        ("mdi mdi-view-dashboard".to_string(), "Layout".to_string()),
        ("mdi mdi-form-textbox".to_string(), "Form".to_string()),
        ("mdi mdi-table".to_string(), "Display".to_string()),
    ];
    
    // Group components by type
    let mut components_by_type: std::collections::HashMap<String, Vec<ComponentInfo>> = 
        std::collections::HashMap::new();
    
    for component in components {
        components_by_type
            .entry(component.component_type.clone())
            .or_insert_with(Vec::new)
            .push(component);
    }

    rsx! {
        div { 
            style: "display: flex; flex-direction: column; height: 100vh; width: 100vw; overflow: hidden;",
            
            // Header
            //div {
            //    style: "
            //        padding: 20px 24px;
            //        background: rgba(255, 255, 255, 0.03);
            //        border-bottom: 1px solid rgba(255, 255, 255, 0.08);
            //        backdrop-filter: blur(10px);
            //    ",
            //    h1 {
            //        style: "
            //            font-size: 28px; 
            //            color: rgba(255, 255, 255, 0.95);
            //            font-weight: 600;
            //            display: flex; 
            //            align-items: center; 
            //            gap: 12px;
            //        ",
            //        i { class: "mdi mdi-view-grid", style: "font-size: 32px;" }
            //        "Component Gallery"
            //    }
            //}
            
            // Gallery content area
            div {
                style: "flex: 1; overflow: hidden; padding: 16px; display: flex;",
                
                GalleryBox {
                    height: "100%",
                    framework_badges: framework_badges,
                    type_badges: type_badges,
                    
                    for (component_type, type_components) in components_by_type.iter() {
                        {
                            // Collect all unique frameworks for this row
                            let mut row_frameworks = vec![];
                            for comp in type_components {
                                for fw in &comp.frameworks {
                                    if !row_frameworks.contains(fw) {
                                        row_frameworks.push(fw.clone());
                                    }
                                }
                            }
                            
                            // Convert framework names to icon classes for row_badges
                            let row_framework_badges: Vec<String> = row_frameworks.iter().map(|fw| {
                                match fw.as_str() {
                                    "React" => "mdi mdi-react",
                                    "Vue" => "mdi mdi-vuejs",
                                    "Angular" => "mdi mdi-angular",
                                    "Svelte" => "mdi mdi-language-javascript",
                                    _ => "mdi mdi-code-tags",
                                }
                            }).map(String::from).collect();
                            
                            // Add type badge for row filtering
                            let type_badge = match component_type.as_str() {
                                "Button" => "mdi mdi-gesture-tap-button",
                                "Layout" => "mdi mdi-view-dashboard",
                                "Form" => "mdi mdi-form-textbox",
                                "Display" => "mdi mdi-table",
                                _ => "mdi mdi-cube",
                            };
                            
                            rsx! {
                                GalleryGridRow {
                                    key: "{component_type}",
                                    min_cell_width: 200,
                                    icon: match component_type.as_str() {
                                        "Button" => "mdi mdi-gesture-tap-button",
                                        "Layout" => "mdi mdi-view-dashboard",
                                        "Form" => "mdi mdi-form-textbox",
                                        "Display" => "mdi mdi-table",
                                        _ => "mdi mdi-cube",
                                    },
                                    label: component_type.clone(),
                                    row_framework_badges: row_framework_badges,
                                    row_type_badge: type_badge.to_string(),
                                    
                                    for component in type_components.iter() {
                                        {
                                            // Convert framework names to icon/label pairs
                                            let badges: Vec<(String, String)> = component.frameworks.iter().map(|fw| {
                                                match fw.as_str() {
                                                    "React" => ("mdi mdi-react".to_string(), "React".to_string()),
                                                    "Vue" => ("mdi mdi-vuejs".to_string(), "Vue".to_string()),
                                                    "Angular" => ("mdi mdi-angular".to_string(), "Angular".to_string()),
                                                    "Svelte" => ("mdi mdi-language-javascript".to_string(), "Svelte".to_string()),
                                                    _ => ("mdi mdi-code-tags".to_string(), fw.clone()),
                                                }
                                            }).collect();
                                            
                                            rsx! {
                                                GalleryCell {
                                                    key: "{component.name}",
                                                    badges: badges,
                                                    
                                                    // Render actual component based on type
                                                    match component.component_type.as_str() {
                                                        "Button" => rsx! {
                                                            button {
                                                                class: "sample-button",
                                                                style: "
                                                                    padding: 10px 20px;
                                                                    background: rgba(59, 130, 246, 0.8);
                                                                    border: 1px solid rgba(59, 130, 246, 0.4);
                                                                    border-radius: 6px;
                                                                    color: white;
                                                                    font-size: 14px;
                                                                    font-weight: 500;
                                                                    cursor: pointer;
                                                                ",
                                                                "Click Me"
                                                            }
                                                        },
                                                        "Layout" => rsx! {
                                                            div {
                                                                style: "
                                                                    display: flex;
                                                                    gap: 8px;
                                                                    padding: 16px;
                                                                    border: 1px solid rgba(255, 255, 255, 0.15);
                                                                    border-radius: 6px;
                                                                ",
                                                                div {
                                                                    style: "
                                                                        width: 40px;
                                                                        height: 40px;
                                                                        background: rgba(59, 130, 246, 0.3);
                                                                        border-radius: 4px;
                                                                    "
                                                                }
                                                                div {
                                                                    style: "
                                                                        width: 40px;
                                                                        height: 40px;
                                                                        background: rgba(139, 92, 246, 0.3);
                                                                        border-radius: 4px;
                                                                    "
                                                                }
                                                                div {
                                                                    style: "
                                                                        width: 40px;
                                                                        height: 40px;
                                                                        background: rgba(236, 72, 153, 0.3);
                                                                        border-radius: 4px;
                                                                    "
                                                                }
                                                            }
                                                        },
                                                        "Form" => rsx! {
                                                            input {
                                                                r#type: "text",
                                                                placeholder: "Enter text...",
                                                                style: "
                                                                    padding: 8px 12px;
                                                                    background: rgba(255, 255, 255, 0.05);
                                                                    border: 1px solid rgba(255, 255, 255, 0.15);
                                                                    border-radius: 6px;
                                                                    color: rgba(255, 255, 255, 0.9);
                                                                    font-size: 14px;
                                                                    outline: none;
                                                                    width: 160px;
                                                                "
                                                            }
                                                        },
                                                        "Display" => rsx! {
                                                            div {
                                                                style: "
                                                                    display: flex;
                                                                    flex-direction: column;
                                                                    gap: 4px;
                                                                    padding: 12px;
                                                                    border: 1px solid rgba(255, 255, 255, 0.15);
                                                                    border-radius: 6px;
                                                                ",
                                                                div {
                                                                    style: "
                                                                        height: 8px;
                                                                        background: rgba(59, 130, 246, 0.6);
                                                                        border-radius: 4px;
                                                                        width: 100%;
                                                                    "
                                                                }
                                                                div {
                                                                    style: "
                                                                        height: 8px;
                                                                        background: rgba(59, 130, 246, 0.4);
                                                                        border-radius: 4px;
                                                                        width: 80%;
                                                                    "
                                                                }
                                                                div {
                                                                    style: "
                                                                        height: 8px;
                                                                        background: rgba(59, 130, 246, 0.3);
                                                                        border-radius: 4px;
                                                                        width: 60%;
                                                                    "
                                                                }
                                                            }
                                                        },
                                                        _ => rsx! {
                                                            div {
                                                                style: "color: rgba(255, 255, 255, 0.5);",
                                                                "Component"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}