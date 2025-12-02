// src/nav_bar.rs
use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    let mut version_open = use_signal(|| false);
    let mut product_open = use_signal(|| false);
    let mut theme_select_open = use_signal(|| false);
    let mut theme_edit_open = use_signal(|| false);
    let mut language_open = use_signal(|| false);
    let mut dark_mode = use_signal(|| true);
    let mut collapsed_open = use_signal(|| false);
    
    let nav = navigator();
    let route = use_route::<Route>();
    
    // Determine active page based on current route
    let is_components_active = matches!(route, Route::ComponentGallery {});
    let is_blocks_active = matches!(route, Route::BlocksGallery {});
    let is_templates_active = matches!(route, Route::TemplatesGallery {});
    
    // Check if we're on any documentation page (any route with sidebar)
    let is_docs_active = !matches!(
        route, 
        Route::ComponentGallery {} | Route::BlocksGallery {} | Route::TemplatesGallery {}
    );

    // Close all dropdowns when clicking outside
    let close_all_dropdowns = move |_| {
        version_open.set(false);
        product_open.set(false);
        theme_select_open.set(false);
        theme_edit_open.set(false);
        language_open.set(false);
        collapsed_open.set(false);
    };

    rsx! {
        // Backdrop to catch outside clicks
        if version_open() || product_open() || theme_select_open() || theme_edit_open() || language_open() || collapsed_open() {
            div { 
                class: "navbar-backdrop",
                onclick: close_all_dropdowns,
            }
        }

        nav { class: "navbar-container",
            div { 
                class: if collapsed_open() { "navbar-content navbar-content-expanded" } else { "navbar-content" },
                
                // Group 1: Logo and Version Dropdowns
                div { class: "navbar-group navbar-brand-group",
                    div { 
                        class: "navbar-logo",
                        onclick: move |_| {
                            nav.push(Route::ComponentsIntro {});
                        },
                        i { class: "mdi mdi-shield-crown" }
                        span { class: "logo-text", "MyApp" }
                    }
                    
                    // Website Version Dropdown
                    div { class: "navbar-dropdown",
                        button { 
                            class: "navbar-dropdown-trigger",
                            onclick: move |e| {
                                e.stop_propagation();
                                version_open.set(!version_open());
                                product_open.set(false);
                                theme_select_open.set(false);
                                theme_edit_open.set(false);
                                language_open.set(false);
                            },
                            i { class: "mdi mdi-tag" }
                            "v2.0.0"
                            i { class: "mdi mdi-chevron-down dropdown-arrow" }
                        }
                        if version_open() {
                            div { 
                                class: "navbar-dropdown-menu",
                                onclick: move |e| e.stop_propagation(),
                                div { class: "navbar-dropdown-item", "v2.0.0 (current)" }
                                div { class: "navbar-dropdown-item", "v1.9.5" }
                                div { class: "navbar-dropdown-item", "v1.9.0" }
                            }
                        }
                    }
                    
                    // Product Version Dropdown
                    div { class: "navbar-dropdown",
                        button { 
                            class: "navbar-dropdown-trigger",
                            onclick: move |e| {
                                e.stop_propagation();
                                product_open.set(!product_open());
                                version_open.set(false);
                                theme_select_open.set(false);
                                theme_edit_open.set(false);
                                language_open.set(false);
                            },
                            i { class: "mdi mdi-package-variant" }
                            "Core"
                            i { class: "mdi mdi-chevron-down dropdown-arrow" }
                        }
                        if product_open() {
                            div { 
                                class: "navbar-dropdown-menu",
                                onclick: move |e| e.stop_propagation(),
                                div { class: "navbar-dropdown-item", "Core (current)" }
                                div { class: "navbar-dropdown-item", "Pro" }
                                div { class: "navbar-dropdown-item", "Enterprise" }
                            }
                        }
                    }
                }

                // Group 2: Search Input
                div { class: "navbar-group navbar-search-group",
                    div { class: "navbar-search",
                        i { class: "mdi mdi-magnify search-icon" }
                        input { 
                            r#type: "text",
                            placeholder: "Search...",
                            class: "search-input"
                        }
                        span { class: "search-shortcut", "⌘K" }
                    }
                }

                // Group 3: Galleries Group + Documentation Button
                div { class: "navbar-group navbar-galleries-group",
                    button { 
                        class: if is_docs_active { "navbar-link active" } else { "navbar-link" },
                        onclick: move |_| {
                            nav.push(Route::ComponentsIntro {});
                        },
                        i { class: "mdi mdi-book-open-variant" }
                        span { "Docs" }
                    }
                    button { 
                        class: if is_components_active { "navbar-link active" } else { "navbar-link" },
                        onclick: move |_| {
                            nav.push(Route::ComponentGallery {});
                        },
                        i { class: "mdi mdi-widgets" }
                        span { "Components" }
                    }
                    button { 
                        class: if is_blocks_active { "navbar-link active" } else { "navbar-link" },
                        onclick: move |_| {
                            nav.push(Route::BlocksGallery {});
                        },
                        i { class: "mdi mdi-view-dashboard" }
                        span { "Blocks" }
                    }
                    button { 
                        class: if is_templates_active { "navbar-link active" } else { "navbar-link" },
                        onclick: move |_| {
                            nav.push(Route::TemplatesGallery {});
                        },
                        i { class: "mdi mdi-application" }
                        span { "Templates" }
                    }
                }

                // Group 4: Themes Group
                div { class: "navbar-group navbar-themes-group",
                    // Theme Select Dropdown
                    div { class: "navbar-dropdown navbar-dropdown-right",
                        button { 
                            class: "navbar-icon-button",
                            onclick: move |e| {
                                e.stop_propagation();
                                theme_select_open.set(!theme_select_open());
                                theme_edit_open.set(false);
                                version_open.set(false);
                                product_open.set(false);
                                language_open.set(false);
                            },
                            title: "Select Theme",
                            i { class: "mdi mdi-palette" }
                        }
                        if theme_select_open() {
                            div { 
                                class: "navbar-dropdown-menu navbar-dropdown-menu-right",
                                onclick: move |e| e.stop_propagation(),
                                div { class: "navbar-dropdown-item", "Default" }
                                div { class: "navbar-dropdown-item", "Midnight" }
                                div { class: "navbar-dropdown-item", "Ocean" }
                                div { class: "navbar-dropdown-item", "Forest" }
                            }
                        }
                    }
                    
                    // Theme Edit Dropdown
                    div { class: "navbar-dropdown navbar-dropdown-right",
                        button { 
                            class: "navbar-icon-button",
                            onclick: move |e| {
                                e.stop_propagation();
                                theme_edit_open.set(!theme_edit_open());
                                theme_select_open.set(false);
                                version_open.set(false);
                                product_open.set(false);
                                language_open.set(false);
                            },
                            title: "Edit Theme",
                            i { class: "mdi mdi-palette-advanced" }
                        }
                        if theme_edit_open() {
                            div { 
                                class: "navbar-dropdown-menu navbar-dropdown-menu-right theme-edit-panel",
                                onclick: move |e| e.stop_propagation(),
                                div { class: "theme-edit-header", "Customize Theme" }
                                div { class: "theme-edit-item",
                                    span { "Primary" }
                                    input { r#type: "color", value: "#3b82f6" }
                                }
                                div { class: "theme-edit-item",
                                    span { "Secondary" }
                                    input { r#type: "color", value: "#8b5cf6" }
                                }
                                div { class: "theme-edit-item",
                                    span { "Accent" }
                                    input { r#type: "color", value: "#ec4899" }
                                }
                            }
                        }
                    }
                    
                    // Dark/Light Switch
                    button { 
                        class: "navbar-icon-button theme-toggle",
                        onclick: move |_| dark_mode.set(!dark_mode()),
                        title: if dark_mode() { "Switch to Light Mode" } else { "Switch to Dark Mode" },
                        i { 
                            class: if dark_mode() { "mdi mdi-weather-night" } else { "mdi mdi-weather-sunny" }
                        }
                    }
                }

                // Group 5: Languages Dropdown
                div { class: "navbar-group navbar-language-group",
                    div { class: "navbar-dropdown navbar-dropdown-right",
                        button { 
                            class: "navbar-icon-button",
                            onclick: move |e| {
                                e.stop_propagation();
                                language_open.set(!language_open());
                                version_open.set(false);
                                product_open.set(false);
                                theme_select_open.set(false);
                                theme_edit_open.set(false);
                            },
                            title: "Select Language",
                            i { class: "mdi mdi-translate" }
                        }
                        if language_open() {
                            div { 
                                class: "navbar-dropdown-menu navbar-dropdown-menu-right",
                                onclick: move |e| e.stop_propagation(),
                                div { class: "navbar-dropdown-item", 
                                    i { class: "mdi mdi-check" }
                                    "English" 
                                }
                                div { class: "navbar-dropdown-item", "Español" }
                                div { class: "navbar-dropdown-item", "Français" }
                                div { class: "navbar-dropdown-item", "Deutsch" }
                                div { class: "navbar-dropdown-item", "日本語" }
                                div { class: "navbar-dropdown-item", "中文" }
                            }
                        }
                    }
                }

                // Collapse/Uncollapse Button (shown only when groups are wrapped)
                div { class: "navbar-collapse-toggle",
                    button {
                        class: "collapse-button",
                        onclick: move |e| {
                            e.stop_propagation();
                            collapsed_open.set(!collapsed_open());
                        },
                        i { 
                            class: if collapsed_open() { "mdi mdi-chevron-up" } else { "mdi mdi-chevron-down" }
                        }
                    }
                }
            }
        }
    }
}