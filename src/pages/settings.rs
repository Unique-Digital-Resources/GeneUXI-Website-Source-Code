// src/pages/settings.rs
use dioxus::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    let mut dark_mode = use_signal(|| false);
    let mut notifications = use_signal(|| true);
    let mut auto_save = use_signal(|| true);

    rsx! {
        div {
            style: "
                padding: 32px;
                color: white;
                max-width: 800px;
                margin: 0 auto;
            ",

            h1 {
                style: "
                    font-size: 36px;
                    margin-bottom: 8px;
                    display: flex;
                    align-items: center;
                    gap: 12px;
                ",
                i { class: "mdi mdi-cog", style: "font-size: 42px;" }
                "Settings"
            }

            p {
                style: "
                    font-size: 16px;
                    opacity: 0.8;
                    margin-bottom: 32px;
                ",
                "Configure your application preferences"
            }

            // Settings sections
            div {
                style: "display: flex; flex-direction: column; gap: 24px;",

                // Appearance Section
                SettingsSection {
                    title: "Appearance",
                    icon: "mdi mdi-palette",

                    SettingsToggle {
                        label: "Dark Mode",
                        description: "Switch between light and dark themes",
                        checked: dark_mode(),
                        on_toggle: move |_| dark_mode.set(!dark_mode())
                    }
                }

                // Notifications Section
                SettingsSection {
                    title: "Notifications",
                    icon: "mdi mdi-bell",

                    SettingsToggle {
                        label: "Enable Notifications",
                        description: "Receive alerts about important events",
                        checked: notifications(),
                        on_toggle: move |_| notifications.set(!notifications())
                    }
                }

                // Editor Section
                SettingsSection {
                    title: "Editor",
                    icon: "mdi mdi-pencil",

                    SettingsToggle {
                        label: "Auto Save",
                        description: "Automatically save changes as you work",
                        checked: auto_save(),
                        on_toggle: move |_| auto_save.set(!auto_save())
                    }
                }

                // About Section
                div {
                    style: "
                        background: rgba(255, 255, 255, 0.1);
                        backdrop-filter: blur(10px);
                        border-radius: 12px;
                        padding: 24px;
                        margin-top: 16px;
                    ",

                    div {
                        style: "display: flex; align-items: center; gap: 12px; margin-bottom: 16px;",
                        i { class: "mdi mdi-information", style: "font-size: 28px; color: #4CAF50;" }
                        h3 { style: "font-size: 20px;", "About" }
                    }

                    div { style: "opacity: 0.9; line-height: 1.6;",
                        p { "Gallery App v1.0.0" }
                        p { style: "margin-top: 8px;", "Built with Dioxus and Rust" }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SettingsSectionProps {
    title: String,
    icon: String,
    children: Element,
}

#[component]
fn SettingsSection(props: SettingsSectionProps) -> Element {
    rsx! {
        div {
            style: "
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(10px);
                border-radius: 12px;
                padding: 24px;
            ",

            div {
                style: "display: flex; align-items: center; gap: 12px; margin-bottom: 20px;",
                i { class: "{props.icon}", style: "font-size: 28px; color: #4CAF50;" }
                h3 { style: "font-size: 20px;", "{props.title}" }
            }

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SettingsToggleProps {
    label: String,
    description: String,
    checked: bool,
    on_toggle: EventHandler<()>,
}

#[component]
fn SettingsToggle(props: SettingsToggleProps) -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 16px;
                background: rgba(255, 255, 255, 0.05);
                border-radius: 8px;
                margin-bottom: 12px;
            ",

            div {
                div {
                    style: "font-size: 16px; font-weight: 600; margin-bottom: 4px;",
                    "{props.label}"
                }
                div {
                    style: "font-size: 14px; opacity: 0.7;",
                    "{props.description}"
                }
            }

            button {
                style: format!(
                    "
                    width: 56px;
                    height: 32px;
                    border-radius: 16px;
                    border: none;
                    cursor: pointer;
                    position: relative;
                    transition: all 0.3s;
                    background: {};
                    ",
                    if props.checked { "#4CAF50" } else { "rgba(255, 255, 255, 0.2)" }
                ),
                onclick: move |_| props.on_toggle.call(()),

                div {
                    style: format!(
                        "
                        width: 24px;
                        height: 24px;
                        border-radius: 50%;
                        background: white;
                        position: absolute;
                        top: 4px;
                        transition: all 0.3s;
                        left: {};
                        ",
                        if props.checked { "28px" } else { "4px" }
                    )
                }
            }
        }
    }
}