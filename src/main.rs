// src/main.rs
use dioxus::prelude::*;

mod sidebar_menu;
use sidebar_menu::{SidebarLayout, MenuItem, MenuSelection};

mod nav_bar;
use nav_bar::NavBar;

mod footer;
use footer::Footer;

mod components;
mod docs;
mod assets;
mod pages;
mod gallery;

use pages::component_gallery::ComponentGalleryPage;
use pages::blocks_gallery::BlocksGalleryPage;
use pages::templates_gallery::TemplatesGalleryPage;

fn main() {
    launch(App);
}

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        // Gallery Routes (top-level)
        #[route("/galleries/components")]
        ComponentGallery {},
        #[route("/galleries/blocks")]
        BlocksGallery {},
        #[route("/galleries/templates")]
        TemplatesGallery {},
        
        // Documentation Routes (with sidebar)
        #[route("/")]
        ComponentsIntro {},
        
        #[route("/components/button")]
        ButtonPage {},
        #[route("/components/card")]
        CardPage {},
        #[route("/components/input")]
        InputPage {},
        
        #[route("/docs")]
        DocsIntro {},
        #[route("/docs/introduction")]
        IntroductionPage {},
        #[route("/docs/getting-started")]
        GettingStartedPage {},
        #[route("/docs/advanced")]
        AdvancedPage {},
        
        #[route("/assets")]
        AssetsIntro {},
        #[route("/assets/images")]
        ImagesPage {},
        #[route("/assets/styles")]
        StylesPage {},
        #[route("/assets/icons")]
        IconsPage {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/@mdi/font@7.4.47/css/materialdesignicons.min.css" }
        //document::Stylesheet { href: asset!("/assets/sidebar_menu.css") }
        //document::Stylesheet { href: asset!("/assets/nav_bar.css") }
        //document::Stylesheet { href: asset!("/assets/gallery_box.css") }
        //document::Stylesheet { href: asset!("/assets/gallery_grid_row.css") }
        //document::Stylesheet { href: asset!("/assets/footer.css") }

				// Inline CSS for now, until i find fix
        document::Style { {include_str!("../assets/sidebar_menu.css")} }
        document::Style { {include_str!("../assets/nav_bar.css")} }
        document::Style { {include_str!("../assets/gallery_box.css")} }
        document::Style { {include_str!("../assets/gallery_grid_row.css")} }
        document::Style { {include_str!("../assets/footer.css")} }

        style { {r#"
            * { margin: 0; padding: 0; box-sizing: border-box; }
            html, body { width: 100%; height: 100%; overflow: hidden; font-family: 'Inter', system-ui, -apple-system, sans-serif; }
            body { background: #0a0b0f; }
            
            .app-layout {
                width: 100%;
                height: 100%;
                display: flex;
                flex-direction: column;
            }
            
            .main-content-wrapper {
                flex: 1;
                margin-top: 72px;
                padding-bottom: 60px;
                overflow-y: auto;
            }
            
            /* Text clipping utilities */
            .text-clip {
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
                max-width: 100%;
            }
            
            .text-clip-2 {
                display: -webkit-box;
                -webkit-line-clamp: 2;
                -webkit-box-orient: vertical;
                overflow: hidden;
                text-overflow: ellipsis;
            }
        "#} }

        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    let route = use_route::<Route>();
    
    // Determine if we should show sidebar based on current route
    let show_sidebar = !matches!(
        route,
        Route::ComponentGallery {} | Route::BlocksGallery {} | Route::TemplatesGallery {}
    );

    rsx! {
        div { class: "app-layout",
            NavBar {}
            
            div { class: "main-content-wrapper",
                if show_sidebar {
                    SidebarLayoutWrapper {}
                } else {
                    // Gallery pages without sidebar
                    Outlet::<Route> {}
                }
            }
            
            Footer {}
        }
    }
}

#[component]
fn SidebarLayoutWrapper() -> Element {
    let nav = navigator();
    
    let handle_menu_select = move |selection: MenuSelection| {
        println!("Menu selected: {} (path: {:?})", selection.selected_id, selection.path);
        
        let _ = match selection.selected_id.as_str() {
            "components" => nav.push(Route::ComponentsIntro {}),
            "components-button" => nav.push(Route::ButtonPage {}),
            "components-card" => nav.push(Route::CardPage {}),
            "components-input" => nav.push(Route::InputPage {}),
            
            "docs" => nav.push(Route::DocsIntro {}),
            "docs-introduction" => nav.push(Route::IntroductionPage {}),
            "docs-getting-started" => nav.push(Route::GettingStartedPage {}),
            "docs-advanced" => nav.push(Route::AdvancedPage {}),
            
            "assets" => nav.push(Route::AssetsIntro {}),
            "assets-images" => nav.push(Route::ImagesPage {}),
            "assets-styles" => nav.push(Route::StylesPage {}),
            "assets-icons" => nav.push(Route::IconsPage {}),
            
            _ => {
                println!("Unknown menu item: {}", selection.selected_id);
                None
            },
        };
    };

    // Components menu tree
    let components_menu = vec![
        MenuItem {
            id: "components".into(),
            label: "Components Overview".into(),
            icon: "mdi mdi-puzzle".into(),
            children: vec![
                MenuItem { 
                    id: "components-button".into(), 
                    label: "Button Component".into(), 
                    icon: "mdi mdi-gesture-tap".into(), 
                    children: vec![],
                },
                MenuItem { 
                    id: "components-card".into(), 
                    label: "Card Component".into(), 
                    icon: "mdi mdi-card".into(),
                    children: vec![],
                },
                MenuItem { 
                    id: "components-input".into(), 
                    label: "Input Component".into(), 
                    icon: "mdi mdi-form-textbox".into(),
                    children: vec![],
                },
            ],
        },
    ];

    // Documentation menu tree
    let documentation_menu = vec![
        MenuItem {
            id: "docs".into(),
            label: "Documentation Home".into(),
            icon: "mdi mdi-book-open".into(),
            children: vec![
                MenuItem { 
                    id: "docs-introduction".into(), 
                    label: "Introduction".into(), 
                    icon: "mdi mdi-book".into(),
                    children: vec![],
                },
                MenuItem { 
                    id: "docs-getting-started".into(), 
                    label: "Getting Started".into(), 
                    icon: "mdi mdi-rocket".into(),
                    children: vec![],
                },
                MenuItem { 
                    id: "docs-advanced".into(), 
                    label: "Advanced Topics".into(), 
                    icon: "mdi mdi-school".into(),
                    children: vec![],
                },
            ],
        },
    ];

    // Assets menu tree
    let assets_menu = vec![
        MenuItem {
            id: "assets".into(),
            label: "Assets Overview".into(),
            icon: "mdi mdi-folder-multiple-image".into(),
            children: vec![
                MenuItem { 
                    id: "assets-images".into(), 
                    label: "Images".into(), 
                    icon: "mdi mdi-image".into(),
                    children: vec![],
                },
                MenuItem { 
                    id: "assets-styles".into(), 
                    label: "Styles".into(), 
                    icon: "mdi mdi-palette".into(),
                    children: vec![],
                },
                MenuItem { 
                    id: "assets-icons".into(), 
                    label: "Icons".into(), 
                    icon: "mdi mdi-emoticon".into(),
                    children: vec![],
                },
            ],
        },
    ];

    let menu_trees = vec![components_menu, documentation_menu, assets_menu];

    // Determine active tab and selected ID based on current route
    let route = use_route::<Route>();
    let (active_tab, selected_id) = match route {
        Route::ComponentsIntro {} => (0, "components".to_string()),
        Route::ButtonPage {} => (0, "components-button".to_string()),
        Route::CardPage {} => (0, "components-card".to_string()),
        Route::InputPage {} => (0, "components-input".to_string()),
        Route::DocsIntro {} => (1, "docs".to_string()),
        Route::IntroductionPage {} => (1, "docs-introduction".to_string()),
        Route::GettingStartedPage {} => (1, "docs-getting-started".to_string()),
        Route::AdvancedPage {} => (1, "docs-advanced".to_string()),
        Route::AssetsIntro {} => (2, "assets".to_string()),
        Route::ImagesPage {} => (2, "assets-images".to_string()),
        Route::StylesPage {} => (2, "assets-styles".to_string()),
        Route::IconsPage {} => (2, "assets-icons".to_string()),
        _ => (0, "components".to_string()),
    };

    rsx! {
        div {
            style: "height: 100%; width: 100%;",
            SidebarLayout {
                menu_trees: menu_trees,
                menu_tabs: vec![
                    "Components".into(), 
                    "Documentation".into(), 
                    "Assets".into()
                ],
                menu_tab_icons: vec![
                    "mdi mdi-puzzle".into(),
                    "mdi mdi-book-open-variant".into(),
                    "mdi mdi-folder-multiple".into()
                ],
                on_select: handle_menu_select,
                active_tab: active_tab,
                selected_id: selected_id,

                Outlet::<Route> {}
            }
        }
    }
}

// ============================================================================
// Gallery Route Components
// ============================================================================

#[component]
fn ComponentGallery() -> Element {
    rsx! { ComponentGalleryPage {} }
}

#[component]
fn BlocksGallery() -> Element {
    rsx! { BlocksGalleryPage {} }
}

#[component]
fn TemplatesGallery() -> Element {
    rsx! { TemplatesGalleryPage {} }
}

// ============================================================================
// Route Components
// ============================================================================

#[component]
fn ComponentsIntro() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-puzzle" }
                "Components Overview"
            }
            p { class: "page-description",
                "Explore our collection of reusable UI components. Each component is built with Dioxus and follows best practices."
            }
            div { class: "content-section",
                h2 { "Available Components" }
                ul {
                    li { "Button - Interactive button component with multiple variants" }
                    li { "Card - Container component for grouping content" }
                    li { "Input - Form input with validation support" }
                }
            }
        }
    }
}

#[component]
fn ButtonPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-gesture-tap" }
                "Button Component"
            }
            p { class: "page-description",
                "A versatile button component with customizable styles and behaviors."
            }
            
            div { class: "content-section",
                h2 { id: "props", "Props" }
                p { "Configure the button with these properties:" }
                pre { code { "label: String\non_click: EventHandler\nvariant: ButtonVariant" } }
            }
            
            div { class: "content-section",
                h2 { id: "examples", "Examples" }
                p { "Basic usage examples:" }
                pre { code { "Button {{ label: \"Click Me\", on_click: handle_click }}" } }
            }
            
            div { class: "content-section",
                h2 { id: "api", "API Reference" }
                p { "Complete API documentation for the Button component." }
            }
        }
    }
}

#[component]
fn CardPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-card" }
                "Card Component"
            }
            p { class: "page-description",
                "A flexible container component for displaying grouped content."
            }
            
            div { class: "content-section",
                h2 { id: "props", "Props" }
                p { "Card component properties and their usage." }
            }
            
            div { class: "content-section",
                h2 { id: "examples", "Examples" }
                p { "See Card in action with practical examples." }
            }
        }
    }
}

#[component]
fn InputPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-form-textbox" }
                "Input Component"
            }
            p { class: "page-description",
                "Form input component with built-in validation and styling." }
            
            div { class: "content-section",
                h2 { id: "props", "Props" }
                p { "Available input properties and configurations." }
            }
            
            div { class: "content-section",
                h2 { id: "validation", "Validation" }
                p { "Learn how to implement form validation with Input." }
            }
        }
    }
}

#[component]
fn DocsIntro() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-book-open" }
                "Documentation Home"
            }
            p { class: "page-description",
                "Welcome to the documentation. Learn everything you need to build amazing applications."
            }
        }
    }
}

#[component]
fn IntroductionPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-book" }
                "Introduction"
            }
            
            div { class: "content-section",
                h2 { id: "overview", "Overview" }
                p { "Get familiar with the framework and its core concepts." }
            }
            
            div { class: "content-section",
                h2 { id: "features", "Key Features" }
                ul {
                    li { "Reactive component system" }
                    li { "Type-safe routing" }
                    li { "Modern CSS support" }
                }
            }
            
            div { class: "content-section",
                h2 { id: "requirements", "Requirements" }
                p { "System requirements and prerequisites for development." }
            }
        }
    }
}

#[component]
fn GettingStartedPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-rocket" }
                "Getting Started"
            }
            
            div { class: "content-section",
                h2 { id: "installation", "Installation" }
                p { "Install the framework and its dependencies:" }
                pre { code { "cargo install dioxus-cli" } }
            }
            
            div { class: "content-section",
                h2 { id: "setup", "Setup" }
                p { "Configure your development environment." }
            }
            
            div { class: "content-section",
                h2 { id: "first-app", "Your First App" }
                p { "Create your first application step by step." }
            }
        }
    }
}

#[component]
fn AdvancedPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-school" }
                "Advanced Topics"
            }
            
            div { class: "content-section",
                h2 { id: "routing", "Routing" }
                p { "Deep dive into advanced routing patterns." }
            }
            
            div { class: "content-section",
                h2 { id: "state", "State Management" }
                p { "Learn complex state management techniques." }
            }
            
            div { class: "content-section",
                h2 { id: "performance", "Performance Optimization" }
                p { "Tips and tricks for optimizing your application." }
            }
        }
    }
}

#[component]
fn AssetsIntro() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-folder-multiple-image" }
                "Assets Overview"
            }
            p { class: "page-description",
                "Learn how to manage and optimize assets in your application."
            }
        }
    }
}

#[component]
fn ImagesPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-image" }
                "Images"
            }
            
            div { class: "content-section",
                h2 { id: "formats", "Supported Formats" }
                p { "PNG, JPG, SVG, WebP and more." }
            }
            
            div { class: "content-section",
                h2 { id: "optimization", "Optimization" }
                p { "Best practices for image optimization." }
            }
        }
    }
}

#[component]
fn StylesPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-palette" }
                "Styles"
            }
            
            div { class: "content-section",
                h2 { id: "css", "CSS Files" }
                p { "Managing CSS assets in your project." }
            }
            
            div { class: "content-section",
                h2 { id: "themes", "Themes" }
                p { "Creating and applying custom themes." }
            }
        }
    }
}

#[component]
fn IconsPage() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { class: "page-title",
                i { class: "mdi mdi-emoticon" }
                "Icons"
            }
            
            div { class: "content-section",
                h2 { id: "mdi", "Material Design Icons" }
                p { "Using MDI icon library in your components." }
            }
            
            div { class: "content-section",
                h2 { id: "custom", "Custom Icons" }
                p { "Adding your own custom icon sets." }
            }
        }
    }
}