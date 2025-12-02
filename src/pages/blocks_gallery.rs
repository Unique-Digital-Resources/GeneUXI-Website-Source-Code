// src/pages/blocks_gallery.rs
use dioxus::prelude::*;
use crate::gallery::gallery_grid_row::{GalleryGridRow, GalleryCell};
use crate::gallery::gallery_box::GalleryBox;

// Sample data structure for blocks
#[derive(Clone, Debug, PartialEq)]
struct BlockInfo {
    name: String,
    block_type: String,
    frameworks: Vec<String>,
}

fn get_sample_blocks() -> Vec<BlockInfo> {
    vec![
        BlockInfo {
            name: "Hero Section".to_string(),
            block_type: "Hero".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        BlockInfo {
            name: "Hero with Image".to_string(),
            block_type: "Hero".to_string(),
            frameworks: vec!["React".to_string(), "Angular".to_string()],
        },
        BlockInfo {
            name: "Feature Grid 3 Col".to_string(),
            block_type: "Features".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        BlockInfo {
            name: "Feature List".to_string(),
            block_type: "Features".to_string(),
            frameworks: vec!["Vue".to_string(), "Svelte".to_string()],
        },
        BlockInfo {
            name: "Pricing Table".to_string(),
            block_type: "Pricing".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string(), "Angular".to_string()],
        },
        BlockInfo {
            name: "Pricing Comparison".to_string(),
            block_type: "Pricing".to_string(),
            frameworks: vec!["React".to_string()],
        },
        BlockInfo {
            name: "Contact Form".to_string(),
            block_type: "Forms".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        BlockInfo {
            name: "Newsletter Signup".to_string(),
            block_type: "Forms".to_string(),
            frameworks: vec!["Svelte".to_string()],
        },
        BlockInfo {
            name: "Footer Links".to_string(),
            block_type: "Footer".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string(), "Angular".to_string()],
        },
        BlockInfo {
            name: "Footer Social".to_string(),
            block_type: "Footer".to_string(),
            frameworks: vec!["React".to_string()],
        },
    ]
}

#[component]
pub fn BlocksGalleryPage() -> Element {
    let blocks = get_sample_blocks();
    
    // Available framework badges for filtering
    let framework_badges = vec![
        ("mdi mdi-react".to_string(), "React".to_string()),
        ("mdi mdi-vuejs".to_string(), "Vue".to_string()),
        ("mdi mdi-angular".to_string(), "Angular".to_string()),
        ("mdi mdi-language-javascript".to_string(), "Svelte".to_string()),
    ];
    
    // Available type badges for filtering
    let type_badges = vec![
        ("mdi mdi-format-header-1".to_string(), "Hero".to_string()),
        ("mdi mdi-star-four-points".to_string(), "Features".to_string()),
        ("mdi mdi-currency-usd".to_string(), "Pricing".to_string()),
        ("mdi mdi-form-textbox".to_string(), "Forms".to_string()),
        ("mdi mdi-page-layout-footer".to_string(), "Footer".to_string()),
    ];
    
    // Group blocks by type
    let mut blocks_by_type: std::collections::HashMap<String, Vec<BlockInfo>> = 
        std::collections::HashMap::new();
    
    for block in blocks {
        blocks_by_type
            .entry(block.block_type.clone())
            .or_insert_with(Vec::new)
            .push(block);
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
            //        i { class: "mdi mdi-view-dashboard", style: "font-size: 32px;" }
            //        "Blocks Gallery"
            //    }
            //}
            
            // Gallery content area
            div {
                style: "flex: 1; overflow: hidden; padding: 16px; display: flex;",
                
                GalleryBox {
                    height: "100%",
                    framework_badges: framework_badges,
                    type_badges: type_badges,
                    
                    for (block_type, type_blocks) in blocks_by_type.iter() {
                        {
                            // Collect all unique frameworks for this row
                            let mut row_frameworks = vec![];
                            for block in type_blocks {
                                for fw in &block.frameworks {
                                    if !row_frameworks.contains(fw) {
                                        row_frameworks.push(fw.clone());
                                    }
                                }
                            }
                            
                            // Convert framework names to icon classes
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
                            let type_badge = match block_type.as_str() {
                                "Hero" => "mdi mdi-format-header-1",
                                "Features" => "mdi mdi-star-four-points",
                                "Pricing" => "mdi mdi-currency-usd",
                                "Forms" => "mdi mdi-form-textbox",
                                "Footer" => "mdi mdi-page-layout-footer",
                                _ => "mdi mdi-cube",
                            };
                            
                            rsx! {
                                GalleryGridRow {
                                    key: "{block_type}",
                                    min_cell_width: 280,
                                    icon: match block_type.as_str() {
                                        "Hero" => "mdi mdi-format-header-1",
                                        "Features" => "mdi mdi-star-four-points",
                                        "Pricing" => "mdi mdi-currency-usd",
                                        "Forms" => "mdi mdi-form-textbox",
                                        "Footer" => "mdi mdi-page-layout-footer",
                                        _ => "mdi mdi-cube",
                                    },
                                    label: block_type.clone(),
                                    row_framework_badges: row_framework_badges,
                                    row_type_badge: type_badge.to_string(),
                                    
                                    for block in type_blocks.iter() {
                                        {
                                            // Convert framework names to icon/label pairs
                                            let badges: Vec<(String, String)> = block.frameworks.iter().map(|fw| {
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
                                                    key: "{block.name}",
                                                    badges: badges,
                                                    
                                                    // Render block preview based on type
                                                    match block.block_type.as_str() {
                                                        "Hero" => rsx! {
                                                            div {
                                                                style: "
                                                                    padding: 24px;
                                                                    display: flex;
                                                                    flex-direction: column;
                                                                    align-items: center;
                                                                    gap: 12px;
                                                                    text-align: center;
                                                                ",
                                                                div {
                                                                    style: "
                                                                        font-size: 18px;
                                                                        font-weight: 700;
                                                                        color: rgba(255, 255, 255, 0.9);
                                                                    ",
                                                                    "Hero Title"
                                                                }
                                                                div {
                                                                    style: "
                                                                        font-size: 12px;
                                                                        color: rgba(255, 255, 255, 0.5);
                                                                    ",
                                                                    "Subtitle text goes here"
                                                                }
                                                                button {
                                                                    style: "
                                                                        margin-top: 8px;
                                                                        padding: 8px 16px;
                                                                        background: rgba(96, 165, 250, 0.8);
                                                                        border: none;
                                                                        border-radius: 6px;
                                                                        color: white;
                                                                        font-size: 12px;
                                                                        cursor: pointer;
                                                                    ",
                                                                    "Call to Action"
                                                                }
                                                            }
                                                        },
                                                        "Features" => rsx! {
                                                            div {
                                                                style: "
                                                                    display: grid;
                                                                    grid-template-columns: repeat(2, 1fr);
                                                                    gap: 12px;
                                                                    padding: 16px;
                                                                ",
                                                                for i in 0..4 {
                                                                    div {
                                                                        style: "
                                                                            padding: 12px;
                                                                            background: rgba(255, 255, 255, 0.03);
                                                                            border: 1px solid rgba(255, 255, 255, 0.1);
                                                                            border-radius: 6px;
                                                                            text-align: center;
                                                                        ",
                                                                        i { 
                                                                            class: "mdi mdi-check-circle",
                                                                            style: "font-size: 20px; color: rgba(96, 165, 250, 0.8);"
                                                                        }
                                                                        div {
                                                                            style: "
                                                                                margin-top: 6px;
                                                                                font-size: 11px;
                                                                                color: rgba(255, 255, 255, 0.7);
                                                                            ",
                                                                            "Feature {i + 1}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        "Pricing" => rsx! {
                                                            div {
                                                                style: "
                                                                    padding: 20px;
                                                                    display: flex;
                                                                    flex-direction: column;
                                                                    align-items: center;
                                                                    gap: 8px;
                                                                ",
                                                                div {
                                                                    style: "
                                                                        font-size: 11px;
                                                                        color: rgba(255, 255, 255, 0.6);
                                                                        text-transform: uppercase;
                                                                        letter-spacing: 1px;
                                                                    ",
                                                                    "Pro Plan"
                                                                }
                                                                div {
                                                                    style: "
                                                                        font-size: 28px;
                                                                        font-weight: 700;
                                                                        color: rgba(255, 255, 255, 0.9);
                                                                    ",
                                                                    "$29"
                                                                }
                                                                div {
                                                                    style: "
                                                                        font-size: 10px;
                                                                        color: rgba(255, 255, 255, 0.4);
                                                                    ",
                                                                    "per month"
                                                                }
                                                                button {
                                                                    style: "
                                                                        margin-top: 12px;
                                                                        padding: 8px 24px;
                                                                        background: rgba(139, 92, 246, 0.8);
                                                                        border: none;
                                                                        border-radius: 6px;
                                                                        color: white;
                                                                        font-size: 12px;
                                                                        cursor: pointer;
                                                                    ",
                                                                    "Subscribe"
                                                                }
                                                            }
                                                        },
                                                        "Forms" => rsx! {
                                                            div {
                                                                style: "
                                                                    padding: 20px;
                                                                    display: flex;
                                                                    flex-direction: column;
                                                                    gap: 10px;
                                                                ",
                                                                input {
                                                                    r#type: "text",
                                                                    placeholder: "Name",
                                                                    style: "
                                                                        padding: 8px 12px;
                                                                        background: rgba(255, 255, 255, 0.05);
                                                                        border: 1px solid rgba(255, 255, 255, 0.15);
                                                                        border-radius: 6px;
                                                                        color: rgba(255, 255, 255, 0.9);
                                                                        font-size: 12px;
                                                                        outline: none;
                                                                    "
                                                                }
                                                                input {
                                                                    r#type: "email",
                                                                    placeholder: "Email",
                                                                    style: "
                                                                        padding: 8px 12px;
                                                                        background: rgba(255, 255, 255, 0.05);
                                                                        border: 1px solid rgba(255, 255, 255, 0.15);
                                                                        border-radius: 6px;
                                                                        color: rgba(255, 255, 255, 0.9);
                                                                        font-size: 12px;
                                                                        outline: none;
                                                                    "
                                                                }
                                                                button {
                                                                    style: "
                                                                        padding: 8px 16px;
                                                                        background: rgba(96, 165, 250, 0.8);
                                                                        border: none;
                                                                        border-radius: 6px;
                                                                        color: white;
                                                                        font-size: 12px;
                                                                        cursor: pointer;
                                                                    ",
                                                                    "Submit"
                                                                }
                                                            }
                                                        },
                                                        "Footer" => rsx! {
                                                            div {
                                                                style: "
                                                                    padding: 16px;
                                                                    display: flex;
                                                                    flex-direction: column;
                                                                    gap: 12px;
                                                                    border-top: 1px solid rgba(255, 255, 255, 0.1);
                                                                ",
                                                                div {
                                                                    style: "
                                                                        display: flex;
                                                                        justify-content: space-around;
                                                                        font-size: 11px;
                                                                        color: rgba(255, 255, 255, 0.6);
                                                                    ",
                                                                    span { "About" }
                                                                    span { "Contact" }
                                                                    span { "Privacy" }
                                                                }
                                                                div {
                                                                    style: "
                                                                        display: flex;
                                                                        justify-content: center;
                                                                        gap: 12px;
                                                                    ",
                                                                    i { class: "mdi mdi-twitter", style: "font-size: 16px; color: rgba(255, 255, 255, 0.5);" }
                                                                    i { class: "mdi mdi-github", style: "font-size: 16px; color: rgba(255, 255, 255, 0.5);" }
                                                                    i { class: "mdi mdi-linkedin", style: "font-size: 16px; color: rgba(255, 255, 255, 0.5);" }
                                                                }
                                                            }
                                                        },
                                                        _ => rsx! {
                                                            div {
                                                                style: "color: rgba(255, 255, 255, 0.5); padding: 20px; text-align: center;",
                                                                "Block Preview"
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