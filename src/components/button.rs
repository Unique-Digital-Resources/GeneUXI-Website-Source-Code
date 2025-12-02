// src/components/button.rs
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

#[component]
pub fn Button(
    label: String,
    #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[props(default)] on_click: Option<EventHandler<()>>,
) -> Element {
    let class = match variant {
        ButtonVariant::Primary => "btn btn-primary",
        ButtonVariant::Secondary => "btn btn-secondary",
        ButtonVariant::Danger => "btn btn-danger",
    };

    rsx! {
        button {
            class: "{class}",
            onclick: move |_| {
                if let Some(handler) = &on_click {
                    handler.call(());
                }
            },
            "{label}"
        }
    }
}