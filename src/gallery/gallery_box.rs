use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct FilterState {
    pub selected_framework_badges: Vec<String>,
    pub selected_type_badges: Vec<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct GalleryBoxProps {
    /// Height of the scrollable area
    #[props(default = String::from("600px"))]
    pub height: String,
    
    /// Framework badge options for filtering (icon class, label)
    #[props(default = vec![])]
    pub framework_badges: Vec<(String, String)>,
    
    /// Type badge options for filtering (icon class, label)
    #[props(default = vec![])]
    pub type_badges: Vec<(String, String)>,
    
    #[props(default = String::new())]
    pub class: String,
    
    pub children: Element,
}

#[component]
pub fn GalleryBox(props: GalleryBoxProps) -> Element {
    // Store selected badges for filtering
    let mut filter_state = use_signal(|| FilterState {
        selected_framework_badges: vec![],
        selected_type_badges: vec![],
    });
    
    // Provide the filter state through context so children can access it
    use_context_provider(|| filter_state);
    
    let mut toggle_framework_badge = move |badge: String| {
        let mut state = filter_state.write();
        if state.selected_framework_badges.contains(&badge) {
            state.selected_framework_badges.retain(|b| b != &badge);
        } else {
            state.selected_framework_badges.push(badge);
        }
    };
    
    let mut toggle_type_badge = move |badge: String| {
        let mut state = filter_state.write();
        if state.selected_type_badges.contains(&badge) {
            state.selected_type_badges.retain(|b| b != &badge);
        } else {
            state.selected_type_badges.push(badge);
        }
    };
    
    rsx! {
        div {
            class: "gallery-box {props.class}",
            style: "
                display: flex;
                flex-direction: column;
                width: 100%;
                height: 100%;
                overflow: hidden;
            ",
            
            // Filter bar
            div {
                class: "filter-bar",
                
                // Filter groups in a single row
                div {
                    class: "filter-bar-row",
                    
                    // Framework filter group
                    if !props.framework_badges.is_empty() {
                        div {
                            class: "filter-group",
                            
                            div {
                                class: "filter-group-label",
                                "Frameworks"
                            }
                            
                            div {
                                class: "filter-group-badges",
                                
                                for (icon, label) in props.framework_badges.iter() {
                                    {
                                        let icon_clone = icon.clone();
                                        let is_selected = filter_state.read().selected_framework_badges.contains(&icon_clone);
                                        
                                        rsx! {
                                            button {
                                                class: if is_selected { "filter-badge active" } else { "filter-badge" },
                                                onclick: move |_| toggle_framework_badge(icon_clone.clone()),
                                                
                                                i {
                                                    class: "{icon} filter-badge-icon",
                                                }
                                                span { 
                                                    class: "filter-badge-text",
                                                    "{label}" 
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Type filter group
                    if !props.type_badges.is_empty() {
                        div {
                            class: "filter-group",
                            
                            div {
                                class: "filter-group-label",
                                "Types"
                            }
                            
                            div {
                                class: "filter-group-badges",
                                
                                for (icon, label) in props.type_badges.iter() {
                                    {
                                        let icon_clone = icon.clone();
                                        let is_selected = filter_state.read().selected_type_badges.contains(&icon_clone);
                                        
                                        rsx! {
                                            button {
                                                class: if is_selected { "filter-badge active" } else { "filter-badge" },
                                                onclick: move |_| toggle_type_badge(icon_clone.clone()),
                                                
                                                i {
                                                    class: "{icon} filter-badge-icon",
                                                }
                                                span { 
                                                    class: "filter-badge-text",
                                                    "{label}" 
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Clear all button
                    if !filter_state.read().selected_framework_badges.is_empty() 
                        || !filter_state.read().selected_type_badges.is_empty() {
                        div {
                            class: "filter-group filter-group-clear",
                            
                            button {
                                class: "filter-badge",
                                onclick: move |_| {
                                    let mut state = filter_state.write();
                                    state.selected_framework_badges.clear();
                                    state.selected_type_badges.clear();
                                },
                                i { class: "mdi mdi-close filter-badge-icon" }
                                span { class: "filter-badge-text", "Clear All" }
                            }
                        }
                    }
                }
            }
            
            // Scrollable content area
            div {
                class: "gallery-content",
                style: "height: {props.height};",
                
                {props.children}
            }
        }
    }
}