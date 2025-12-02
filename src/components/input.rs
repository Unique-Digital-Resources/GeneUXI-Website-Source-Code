// src/components/input.rs
use dioxus::prelude::*;

#[component]
pub fn Input(
    label: String,
    value: String,
    #[props(default = "text".to_string())] input_type: String,
    #[props(default)] on_change: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        div { class: "input-group",
            label { class: "input-label", "{label}" }
            input {
                class: "input-field",
                r#type: "{input_type}",
                value: "{value}",
                oninput: move |evt| {
                    if let Some(handler) = &on_change {
                        handler.call(evt.value());
                    }
                }
            }
        }
    }
}