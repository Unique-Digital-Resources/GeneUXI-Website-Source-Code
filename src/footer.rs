// src/footer.rs
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer-container",
            div { class: "footer-content",
                // Brand
                div { class: "footer-brand",
                    i { class: "mdi mdi-shield-crown" }
                    span { class: "footer-brand-text", "MyApp" }
                }

                // Quick Links
                div { class: "footer-column",
                    a { class: "footer-link", href: "#", "Features" }
                    a { class: "footer-link", href: "#", "Docs" }
                    a { class: "footer-link", href: "#", "Components" }
                    a { class: "footer-link", href: "#", "Support" }
                }

                // Social Icons
                div { class: "footer-social",
                    a { class: "footer-social-link", href: "#", title: "GitHub",
                        i { class: "mdi mdi-github" }
                    }
                    a { class: "footer-social-link", href: "#", title: "YouTube",
                        i { class: "mdi mdi-youtube" }
                    }
                    a { class: "footer-social-link", href: "#", title: "Twitter",
                        i { class: "mdi mdi-twitter" }
                    }
                    a { class: "footer-social-link", href: "#", title: "Discord",
                        i { class: "mdi mdi-discord" }
                    }
                }
            }
        }
    }
}