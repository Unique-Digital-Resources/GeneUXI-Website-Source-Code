use dioxus::prelude::*;

// Import FilterState from gallery_box module
use super::gallery_box::FilterState;

#[derive(Props, Clone, PartialEq)]
pub struct GalleryCellProps {
    #[props(default = String::new())]
    pub class: String,
    
    #[props(default = vec![])]
    pub badges: Vec<(String, String)>,
    
    pub children: Element,
}

#[component]
pub fn GalleryCell(props: GalleryCellProps) -> Element {
    let filter_state = use_context::<Signal<FilterState>>();
    
    let is_visible = {
        let state = filter_state.read();
        if state.selected_framework_badges.is_empty() {
            true
        } else {
            props.badges.iter().any(|(icon, _)| state.selected_framework_badges.contains(icon))
        }
    };
    
    if !is_visible {
        return rsx! { div { class: "gallery-cell-hidden" } };
    }
    
    rsx! {
        div {
            class: "gallery-cell {props.class}",
            
            // Inner centered panel with padding
            div {
                class: "gallery-cell-inner",
                
                div {
                    class: "gallery-cell-content",
                    {props.children}
                }
            }
            
            // Top-right badges
            if !props.badges.is_empty() {
                div {
                    class: "gallery-cell-badges",
                    
                    for (icon, label) in props.badges.iter() {
                        div {
                            class: "gallery-cell-badge",
                            title: "{label}",
                            i {
                                class: "{icon} gallery-cell-badge-icon",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct GalleryGridRowProps {
    #[props(default = 200)]
    pub min_cell_width: usize,
    
    #[props(default = String::from("mdi mdi-cube"))]
    pub icon: String,
    
    #[props(default = String::from("Type"))]
    pub label: String,
    
    #[props(default = String::new())]
    pub class: String,
    
    #[props(default = vec![])]
    pub row_framework_badges: Vec<String>,
    
    #[props(default = String::new())]
    pub row_type_badge: String,
    
    pub children: Element,
}

#[component]
pub fn GalleryGridRow(props: GalleryGridRowProps) -> Element {
    let min_width = props.min_cell_width;
    
    let filter_state = use_context::<Signal<FilterState>>();
    
    let is_row_visible = {
        let state = filter_state.read();
        
        // Check framework filter
        let framework_matches = if state.selected_framework_badges.is_empty() {
            true
        } else {
            props.row_framework_badges.iter().any(|badge| state.selected_framework_badges.contains(badge))
        };
        
        // Check type filter
        let type_matches = if state.selected_type_badges.is_empty() {
            true
        } else {
            state.selected_type_badges.contains(&props.row_type_badge)
        };
        
        framework_matches && type_matches
    };
    
    if !is_row_visible {
        return rsx! { div { style: "display: none;" } };
    }
    
    rsx! {
        div {
            class: "gallery-grid-row {props.class}",
            
            // Type panel on the left (fixed width)
            div {
                class: "type-panel",
                
                i {
                    class: "{props.icon} type-panel-icon",
                }
                
                span {
                    class: "type-panel-label text-clip",
                    "{props.label}"
                }
            }
            
            // Flex container for cells with wrapping
            div {
                class: "gallery-cells-container",
                
                {props.children}
            }
        }
        
        style {
            "
                .gallery-cell {{
                    flex: 1 1 {min_width}px;
                }}
            "
        }
    }
}