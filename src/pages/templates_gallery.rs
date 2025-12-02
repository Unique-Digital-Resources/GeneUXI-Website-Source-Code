// src/pages/templates_gallery.rs
use dioxus::prelude::*;
use crate::gallery::gallery_grid_row::{GalleryGridRow, GalleryCell};
use crate::gallery::gallery_box::GalleryBox;

// Sample data structure for templates
#[derive(Clone, Debug, PartialEq)]
struct TemplateInfo {
    name: String,
    template_type: String,
    frameworks: Vec<String>,
}

fn get_sample_templates() -> Vec<TemplateInfo> {
    vec![
        TemplateInfo {
            name: "Dashboard Admin".to_string(),
            template_type: "Dashboard".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        TemplateInfo {
            name: "Analytics Dashboard".to_string(),
            template_type: "Dashboard".to_string(),
            frameworks: vec!["React".to_string()],
        },
        TemplateInfo {
            name: "E-commerce Store".to_string(),
            template_type: "E-commerce".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        TemplateInfo {
            name: "Product Catalog".to_string(),
            template_type: "E-commerce".to_string(),
            frameworks: vec!["Vue".to_string(), "Angular".to_string()],
        },
        TemplateInfo {
            name: "Landing Page".to_string(),
            template_type: "Marketing".to_string(),
            frameworks: vec!["React".to_string(), "Svelte".to_string()],
        },
        TemplateInfo {
            name: "Product Launch".to_string(),
            template_type: "Marketing".to_string(),
            frameworks: vec!["React".to_string()],
        },
        TemplateInfo {
            name: "Blog Platform".to_string(),
            template_type: "Content".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        TemplateInfo {
            name: "Documentation Site".to_string(),
            template_type: "Content".to_string(),
            frameworks: vec!["Vue".to_string(), "Svelte".to_string()],
        },
        TemplateInfo {
            name: "Portfolio Site".to_string(),
            template_type: "Portfolio".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string(), "Angular".to_string()],
        },
        TemplateInfo {
            name: "Agency Portfolio".to_string(),
            template_type: "Portfolio".to_string(),
            frameworks: vec!["React".to_string()],
        },
        TemplateInfo {
            name: "SaaS Application".to_string(),
            template_type: "SaaS".to_string(),
            frameworks: vec!["React".to_string(), "Vue".to_string()],
        },
        TemplateInfo {
            name: "Multi-tenant App".to_string(),
            template_type: "SaaS".to_string(),
            frameworks: vec!["React".to_string(), "Angular".to_string()],
        },
    ]
}

#[component]
pub fn TemplatesGalleryPage() -> Element {
    let templates = get_sample_templates();
    
    // Available framework badges for filtering
    let framework_badges = vec![
        ("mdi mdi-react".to_string(), "React".to_string()),
        ("mdi mdi-vuejs".to_string(), "Vue".to_string()),
        ("mdi mdi-angular".to_string(), "Angular".to_string()),
        ("mdi mdi-language-javascript".to_string(), "Svelte".to_string()),
    ];
    
    // Available type badges for filtering
    let type_badges = vec![
        ("mdi mdi-view-dashboard".to_string(), "Dashboard".to_string()),
        ("mdi mdi-cart".to_string(), "E-commerce".to_string()),
        ("mdi mdi-bullhorn".to_string(), "Marketing".to_string()),
        ("mdi mdi-file-document".to_string(), "Content".to_string()),
        ("mdi mdi-briefcase".to_string(), "Portfolio".to_string()),
        ("mdi mdi-cloud".to_string(), "SaaS".to_string()),
    ];
    
    // Group templates by type
    let mut templates_by_type: std::collections::HashMap<String, Vec<TemplateInfo>> = 
        std::collections::HashMap::new();
    
    for template in templates {
        templates_by_type
            .entry(template.template_type.clone())
            .or_insert_with(Vec::new)
            .push(template);
    }

    rsx! {
        div { 
            //style: "display: flex; flex-direction: column; height: 100vh; width: 100vw; overflow: hidden;",
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
            //        i { class: "mdi mdi-application", style: "font-size: 32px;" }
            //        "Templates Gallery"
            //    }
            //}
            
            // Gallery content area
            div {
                style: "flex: 1; overflow: hidden; padding: 16px; display: flex;",
                
                GalleryBox {
                    height: "100%",
                    framework_badges: framework_badges,
                    type_badges: type_badges,
                    
                    for (template_type, type_templates) in templates_by_type.iter() {
                        {
                            // Collect all unique frameworks for this row
                            let mut row_frameworks = vec![];
                            for template in type_templates {
                                for fw in &template.frameworks {
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
                            let type_badge = match template_type.as_str() {
                                "Dashboard" => "mdi mdi-view-dashboard",
                                "E-commerce" => "mdi mdi-cart",
                                "Marketing" => "mdi mdi-bullhorn",
                                "Content" => "mdi mdi-file-document",
                                "Portfolio" => "mdi mdi-briefcase",
                                "SaaS" => "mdi mdi-cloud",
                                _ => "mdi mdi-application",
                            };
                            
                            rsx! {
                                GalleryGridRow {
                                    key: "{template_type}",
                                    min_cell_width: 320,
                                    icon: match template_type.as_str() {
                                        "Dashboard" => "mdi mdi-view-dashboard",
                                        "E-commerce" => "mdi mdi-cart",
                                        "Marketing" => "mdi mdi-bullhorn",
                                        "Content" => "mdi mdi-file-document",
                                        "Portfolio" => "mdi mdi-briefcase",
                                        "SaaS" => "mdi mdi-cloud",
                                        _ => "mdi mdi-application",
                                    },
                                    label: template_type.clone(),
                                    row_framework_badges: row_framework_badges,
                                    row_type_badge: type_badge.to_string(),
                                    
                                    for template in type_templates.iter() {
                                        {
                                            // Convert framework names to icon/label pairs
                                            let badges: Vec<(String, String)> = template.frameworks.iter().map(|fw| {
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
                                                    key: "{template.name}",
                                                    badges: badges,
                                                    
                                                    // Render template preview based on type
                                                    match template.template_type.as_str() {
                                                        "Dashboard" => rsx! {
                                                            div {
                                                                style: "
                                                                    display: grid;
                                                                    grid-template-columns: 1fr 1fr;
                                                                    grid-template-rows: 60px 1fr 1fr;
                                                                    gap: 8px;
                                                                    padding: 12px;
                                                                    height: 200px;
                                                                ",
                                                                // Header
                                                                div {
                                                                    style: "
                                                                        grid-column: 1 / -1;
                                                                        background: rgba(96, 165, 250, 0.2);
                                                                        border-radius: 4px;
                                                                        display: flex;
                                                                        align-items: center;
                                                                        padding: 0 12px;
                                                                        gap: 8px;
                                                                    ",
                                                                    i { class: "mdi mdi-menu", style: "font-size: 16px; color: rgba(255, 255, 255, 0.6);" }
                                                                    div { style: "flex: 1; height: 20px; background: rgba(255, 255, 255, 0.1); border-radius: 3px;" }
                                                                }
                                                                // Stats cards
                                                                div {
                                                                    style: "
                                                                        background: rgba(139, 92, 246, 0.15);
                                                                        border: 1px solid rgba(139, 92, 246, 0.3);
                                                                        border-radius: 4px;
                                                                        padding: 8px;
                                                                    ",
                                                                    div { style: "font-size: 18px; font-weight: 700; color: rgba(255, 255, 255, 0.9);", "1,234" }
                                                                    div { style: "font-size: 10px; color: rgba(255, 255, 255, 0.5);", "Users" }
                                                                }
                                                                div {
                                                                    style: "
                                                                        background: rgba(236, 72, 153, 0.15);
                                                                        border: 1px solid rgba(236, 72, 153, 0.3);
                                                                        border-radius: 4px;
                                                                        padding: 8px;
                                                                    ",
                                                                    div { style: "font-size: 18px; font-weight: 700; color: rgba(255, 255, 255, 0.9);", "$45k" }
                                                                    div { style: "font-size: 10px; color: rgba(255, 255, 255, 0.5);", "Revenue" }
                                                                }
                                                                // Chart area
                                                                div {
                                                                    style: "
                                                                        grid-column: 1 / -1;
                                                                        background: rgba(255, 255, 255, 0.05);
                                                                        border: 1px solid rgba(255, 255, 255, 0.1);
                                                                        border-radius: 4px;
                                                                        display: flex;
                                                                        align-items: flex-end;
                                                                        padding: 8px;
                                                                        gap: 4px;
                                                                    ",
                                                                    for height in [40, 60, 45, 75, 55, 80, 65] {
                                                                        div {
                                                                            style: "
                                                                                flex: 1;
                                                                                height: {height}%;
                                                                                background: rgba(96, 165, 250, 0.6);
                                                                                border-radius: 2px;
                                                                            "
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        "E-commerce" => rsx! {
                                                            div {
                                                                style: "
                                                                    display: grid;
                                                                    grid-template-columns: repeat(2, 1fr);
                                                                    gap: 10px;
                                                                    padding: 16px;
                                                                ",
                                                                for i in 0..4 {
                                                                    div {
                                                                        style: "
                                                                            background: rgba(255, 255, 255, 0.03);
                                                                            border: 1px solid rgba(255, 255, 255, 0.1);
                                                                            border-radius: 6px;
                                                                            overflow: hidden;
                                                                        ",
                                                                        div {
                                                                            style: "
                                                                                height: 60px;
                                                                                background: rgba(96, 165, 250, 0.2);
                                                                                display: flex;
                                                                                align-items: center;
                                                                                justify-content: center;
                                                                            ",
                                                                            i { class: "mdi mdi-image", style: "font-size: 24px; color: rgba(255, 255, 255, 0.3);" }
                                                                        }
                                                                        div {
                                                                            style: "padding: 8px;",
                                                                            div { style: "font-size: 11px; color: rgba(255, 255, 255, 0.7); margin-bottom: 4px;", "Product {i + 1}" }
                                                                            div { style: "font-size: 13px; font-weight: 600; color: rgba(96, 165, 250, 0.9);", "$99" }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        "Marketing" => rsx! {
                                                            div {
                                                                style: "
                                                                    padding: 20px;
                                                                    display: flex;
                                                                    flex-direction: column;
                                                                    align-items: center;
                                                                    gap: 12px;
                                                                    text-align: center;
                                                                ",
                                                                i { 
                                                                    class: "mdi mdi-rocket-launch",
                                                                    style: "font-size: 40px; color: rgba(236, 72, 153, 0.8);"
                                                                }
                                                                div {
                                                                    style: "
                                                                        font-size: 16px;
                                                                        font-weight: 700;
                                                                        color: rgba(255, 255, 255, 0.9);
                                                                    ",
                                                                    "Launch Your Product"
                                                                }
                                                                div {
                                                                    style: "
                                                                        font-size: 11px;
                                                                        color: rgba(255, 255, 255, 0.5);
                                                                        line-height: 1.4;
                                                                    ",
                                                                    "Beautiful landing page template"
                                                                }
                                                                button {
                                                                    style: "
                                                                        padding: 8px 20px;
                                                                        background: linear-gradient(135deg, rgba(236, 72, 153, 0.8), rgba(139, 92, 246, 0.8));
                                                                        border: none;
                                                                        border-radius: 6px;
                                                                        color: white;
                                                                        font-size: 12px;
                                                                        cursor: pointer;
                                                                    ",
                                                                    "Get Started"
                                                                }
                                                            }
                                                        },
                                                        "Content" => rsx! {
                                                            div {
                                                                style: "
                                                                    padding: 16px;
                                                                    display: flex;
                                                                    flex-direction: column;
                                                                    gap: 10px;
                                                                ",
                                                                div {
                                                                    style: "
                                                                        height: 12px;
                                                                        background: rgba(96, 165, 250, 0.6);
                                                                        border-radius: 3px;
                                                                        width: 70%;
                                                                    "
                                                                }
                                                                for width in [100, 95, 90, 85, 75] {
                                                                    div {
                                                                        style: "
                                                                            height: 6px;
                                                                            background: rgba(255, 255, 255, 0.2);
                                                                            border-radius: 2px;
                                                                            width: {width}%;
                                                                        "
                                                                    }
                                                                }
                                                                div {
                                                                    style: "
                                                                        margin-top: 8px;
                                                                        height: 8px;
                                                                        background: rgba(139, 92, 246, 0.5);
                                                                        border-radius: 3px;
                                                                        width: 50%;
                                                                    "
                                                                }
                                                            }
                                                        },
                                                        "Portfolio" => rsx! {
                                                            div {
                                                                style: "
                                                                    display: grid;
                                                                    grid-template-columns: repeat(3, 1fr);
                                                                    gap: 8px;
                                                                    padding: 12px;
                                                                ",
                                                                for (idx, color) in ["96, 165, 250", "139, 92, 246", "236, 72, 153", "34, 197, 94", "251, 146, 60", "14, 165, 233"].iter().enumerate() {
                                                                    div {
                                                                        style: "
                                                                            aspect-ratio: 1;
                                                                            background: rgba({color}, 0.3);
                                                                            border: 1px solid rgba({color}, 0.5);
                                                                            border-radius: 4px;
                                                                            display: flex;
                                                                            align-items: center;
                                                                            justify-content: center;
                                                                        ",
                                                                        i { 
                                                                            class: "mdi mdi-image",
                                                                            style: "font-size: 20px; color: rgba({color}, 0.8);"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        "SaaS" => rsx! {
                                                            div {
                                                                style: "
                                                                    padding: 16px;
                                                                    display: flex;
                                                                    flex-direction: column;
                                                                    gap: 8px;
                                                                ",
                                                                // Header bar
                                                                div {
                                                                    style: "
                                                                        display: flex;
                                                                        justify-content: space-between;
                                                                        align-items: center;
                                                                        padding: 8px;
                                                                        background: rgba(96, 165, 250, 0.15);
                                                                        border-radius: 4px;
                                                                    ",
                                                                    i { class: "mdi mdi-cloud", style: "font-size: 18px; color: rgba(96, 165, 250, 0.8);" }
                                                                    i { class: "mdi mdi-account-circle", style: "font-size: 18px; color: rgba(255, 255, 255, 0.6);" }
                                                                }
                                                                // Content cards
                                                                for i in 0..3 {
                                                                    div {
                                                                        style: "
                                                                            padding: 10px;
                                                                            background: rgba(255, 255, 255, 0.03);
                                                                            border: 1px solid rgba(255, 255, 255, 0.1);
                                                                            border-radius: 4px;
                                                                            display: flex;
                                                                            align-items: center;
                                                                            gap: 8px;
                                                                        ",
                                                                        div {
                                                                            style: "
                                                                                width: 32px;
                                                                                height: 32px;
                                                                                background: rgba(139, 92, 246, 0.3);
                                                                                border-radius: 4px;
                                                                            "
                                                                        }
                                                                        div {
                                                                            style: "flex: 1;",
                                                                            div { style: "height: 6px; background: rgba(255, 255, 255, 0.3); border-radius: 2px; width: 80%; margin-bottom: 4px;" }
                                                                            div { style: "height: 4px; background: rgba(255, 255, 255, 0.2); border-radius: 2px; width: 50%;" }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        _ => rsx! {
                                                            div {
                                                                style: "
                                                                    padding: 20px;
                                                                    display: flex;
                                                                    align-items: center;
                                                                    justify-content: center;
                                                                    color: rgba(255, 255, 255, 0.5);
                                                                ",
                                                                "Template Preview"
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