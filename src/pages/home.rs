// src/pages/home.rs
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100%;
                color: white;
                text-align: center;
                padding: 32px;
            ",

            div {
                style: "
                    background: rgba(255, 255, 255, 0.1);
                    backdrop-filter: blur(10px);
                    border-radius: 16px;
                    padding: 48px;
                    max-width: 600px;
                ",

                i {
                    class: "mdi mdi-home-circle",
                    style: "font-size: 96px; color: #4CAF50; margin-bottom: 24px;"
                }

                h1 {
                    style: "font-size: 48px; margin-bottom: 16px; font-weight: 700;",
                    "Welcome Home"
                }

                p {
                    style: "font-size: 18px; line-height: 1.6; opacity: 0.9; margin-bottom: 32px;",
                    "This is your home page. Use the navigation menu on the left or the toolbar above to explore different sections of the application."
                }

                div {
                    style: "display: flex; gap: 16px; justify-content: center; flex-wrap: wrap;",

                    div {
                        style: "
                            background: rgba(255, 255, 255, 0.15);
                            padding: 24px;
                            border-radius: 12px;
                            flex: 1;
                            min-width: 150px;
                        ",
                        i { class: "mdi mdi-image-multiple", style: "font-size: 32px; display: block; margin-bottom: 8px;" }
                        strong { "Gallery" }
                        div { style: "font-size: 14px; opacity: 0.8; margin-top: 4px;", "View images" }
                    }

                    div {
                        style: "
                            background: rgba(255, 255, 255, 0.15);
                            padding: 24px;
                            border-radius: 12px;
                            flex: 1;
                            min-width: 150px;
                        ",
                        i { class: "mdi mdi-folder", style: "font-size: 32px; display: block; margin-bottom: 8px;" }
                        strong { "Projects" }
                        div { style: "font-size: 14px; opacity: 0.8; margin-top: 4px;", "Manage files" }
                    }

                    div {
                        style: "
                            background: rgba(255, 255, 255, 0.15);
                            padding: 24px;
                            border-radius: 12px;
                            flex: 1;
                            min-width: 150px;
                        ",
                        i { class: "mdi mdi-cog", style: "font-size: 32px; display: block; margin-bottom: 8px;" }
                        strong { "Settings" }
                        div { style: "font-size: 14px; opacity: 0.8; margin-top: 4px;", "Configure app" }
                    }
                }
            }
        }
    }
}