// src/components/card.rs
use dioxus::prelude::*;

#[component]
pub fn Card(title: String, children: Element) -> Element {
    rsx! {
        div { class: "card",
            div { class: "card-header",
                h3 { "{title}" }
            }
            div { class: "card-body",
                {children}
            }
        }
    }
}