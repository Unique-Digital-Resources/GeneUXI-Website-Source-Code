use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub children: Vec<MenuItem>,
}

#[derive(Clone, PartialEq)]
pub struct MenuSelection {
    pub selected_id: String,
    pub path: Vec<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct SidebarLayoutProps {
    pub children: Element,
    pub menu_trees: Vec<Vec<MenuItem>>,
    #[props(default = vec!["Menu 1".to_string(), "Menu 2".to_string(), "Menu 3".to_string()])]
    pub menu_tabs: Vec<String>,
    #[props(default = vec![])]
    pub menu_tab_icons: Vec<String>,
    #[props(default = None)]
    pub on_select: Option<EventHandler<MenuSelection>>,
    pub active_tab: usize,
    pub selected_id: String,
}

#[component]
pub fn SidebarLayout(props: SidebarLayoutProps) -> Element {
    let mut sidebar_width = use_signal(|| 240.0);
    let mut is_dragging = use_signal(|| false);
    let mut is_collapsed = use_signal(|| false);
    let mut expanded_items = use_signal(|| std::collections::HashSet::<String>::new());
    let mut selected_item = use_signal(|| props.selected_id.clone());

    // Use props directly for active_tab and selected_item - no internal state
    let current_selected = &props.selected_id;
    let current_tab = props.active_tab;
    
    // Update selected_item signal to match props
    if selected_item.read().as_str() != current_selected.as_str() {
        selected_item.set(current_selected.clone());
    }
    
    // Auto-expand parent items based on current selection
    {
        let current_selected = props.selected_id.clone();
        let current_tab = props.active_tab;
        let menu_trees = props.menu_trees.clone();
        
        use_effect(move || {
            let mut expanded = expanded_items.write();
            
            // Clear and rebuild expanded items based on current selection
            expanded.clear();
            
            if current_tab < menu_trees.len() {
                for item in &menu_trees[current_tab] {
                    if item.id == current_selected {
                        // Top-level item selected - expand it
                        expanded.insert(item.id.clone());
                    } else if !item.children.is_empty() {
                        // Check children
                        for child in &item.children {
                            if child.id == current_selected {
                                // Child is selected - expand parent
                                expanded.insert(item.id.clone());
                            } else if !child.children.is_empty() {
                                // Check nested children
                                for nested in &child.children {
                                    if nested.id == current_selected {
                                        // Nested child is selected - expand both parent and child
                                        expanded.insert(item.id.clone());
                                        expanded.insert(child.id.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    let min_width = 180.0;
    let collapse_width = 120.0;

    let handle_mouse_down = move |_| is_dragging.set(true);
    let handle_mouse_move = move |evt: Event<MouseData>| {
        if is_dragging() {
            let new_width = evt.page_coordinates().x as f64;
            if new_width < collapse_width {
                is_collapsed.set(true);
                sidebar_width.set(0.0);
            } else if new_width >= min_width {
                is_collapsed.set(false);
                sidebar_width.set(new_width.min(400.0));
            }
        }
    };
    let handle_mouse_up = move |_| is_dragging.set(false);

    let toggle_menu = move |_| {
        if is_collapsed() {
            is_collapsed.set(false);
            sidebar_width.set(240.0);
        } else {
            is_collapsed.set(true);
            sidebar_width.set(0.0);
        }
    };

    let current_width = sidebar_width();

    let current_menu = if !props.menu_trees.is_empty() && current_tab < props.menu_trees.len() {
        &props.menu_trees[current_tab]
    } else if !props.menu_trees.is_empty() {
        &props.menu_trees[0]
    } else {
        &vec![]
    };

    rsx! {
        div {
            class: "sidebar-layout",
            onmousemove: handle_mouse_move,
            onmouseup: handle_mouse_up,

            if !is_collapsed() {
                div {
                    class: "sidebar",
                    style: "width: {current_width}px;",

                    div { class: "sidebar-inner",
                        div { class: "sidebar-tabs",
                            for (idx, tab) in props.menu_tabs.iter().enumerate() {
                                {
                                    let icon = props.menu_tab_icons.get(idx).cloned().unwrap_or_default();
                                    let on_select = props.on_select.clone();
                                    let menu_trees = props.menu_trees.clone();
                                    
                                    rsx! {
                                        button {
                                            class: if current_tab == idx { "sidebar-tab active" } else { "sidebar-tab" },
                                            onclick: move |_| {
                                                // When clicking a tab, navigate to its root item
                                                if let Some(handler) = &on_select {
                                                    if idx < menu_trees.len() && !menu_trees[idx].is_empty() {
                                                        let root_item = &menu_trees[idx][0];
                                                        handler.call(MenuSelection {
                                                            selected_id: root_item.id.clone(),
                                                            path: vec![root_item.label.clone()],
                                                        });
                                                    }
                                                }
                                            },
                                            if !icon.is_empty() {
                                                i { class: "{icon}", style: "margin-right: 8px;" }
                                            }
                                            "{tab}"
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "sidebar-menu-content",
                            div { class: "menu-root-list",
                                for item in current_menu.iter() {
                                    TreeNode {
                                        item: item.clone(),
                                        level: 0,
                                        expanded_items: expanded_items,
                                        selected_item: selected_item,
                                        on_select: props.on_select.clone()
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Toggle button
            {
                let button_left = if is_collapsed() { 0.0 } else { current_width };
                let button_class = if is_collapsed() { "sidebar-toggle-button collapsed" } else { "sidebar-toggle-button" };

                rsx! {
                    button {
                        class: "{button_class}",
                        style: "left: {button_left}px;",
                        onclick: toggle_menu,
                        i {
                            class: if is_collapsed() {
                                "mdi mdi-chevron-right sidebar-toggle-icon"
                            } else {
                                "mdi mdi-chevron-left sidebar-toggle-icon"
                            },
                        }
                    }
                }
            }

            if !is_collapsed() {
                div {
                    class: "sidebar-splitter",
                    onmousedown: handle_mouse_down,
                    div { class: "sidebar-splitter-hitbox" }
                }
            }

            div {
                class: "main-content-area",
                {props.children}
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TreeNode – All items are now selectable navigation items
// ────────────────────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct TreeNodeProps {
    item: MenuItem,
    level: i32,
    expanded_items: Signal<std::collections::HashSet<String>>,
    selected_item: Signal<String>,
    #[props(default = None)]
    on_select: Option<EventHandler<MenuSelection>>,
}

#[component]
fn TreeNode(mut props: TreeNodeProps) -> Element {
    let is_expanded = props.expanded_items.read().contains(&props.item.id);
    let is_selected = props.selected_item.read().clone() == props.item.id;
    let has_children = !props.item.children.is_empty();

    let item_id = props.item.id.clone();
    let item_id_for_toggle = item_id.clone();
    let item_id_for_select = item_id.clone();
    let item_label = props.item.label.clone();

    let mut toggle_expand = move |_| {
        let mut set = props.expanded_items.write();
        if set.contains(&item_id_for_toggle) {
            set.remove(&item_id_for_toggle);
        } else {
            set.insert(item_id_for_toggle.clone());
        }
    };

    let mut handle_select = move |_| {
        props.selected_item.set(item_id_for_select.clone());
        
        if let Some(handler) = &props.on_select {
            handler.call(MenuSelection {
                selected_id: item_id_for_select.clone(),
                path: vec![item_label.clone()],
            });
        }
    };

    // ANY item with children gets rendered as a panel
    if has_children {
        rsx! {
            div { class: "menu-panel",
                // Header is now both expandable AND selectable
                div {
                    class: if is_selected { "menu-panel-header selected" } else { "menu-panel-header" },
                    
                    // Left side - expand/collapse icon
                    i {
                        class: if is_expanded {
                            "mdi mdi-chevron-down menu-panel-chevron"
                        } else {
                            "mdi mdi-chevron-right menu-panel-chevron"
                        },
                        onclick: move |evt| {
                            evt.stop_propagation();
                            toggle_expand(());
                        },
                        style: "cursor: pointer; padding: 4px; margin-right: 4px;",
                    }

                    // Middle - icon and title (clicking selects this item)
                    div {
                        style: "display: flex; align-items: center; gap: 12px; flex: 1; cursor: pointer;",
                        onclick: handle_select,

                        i { class: "{props.item.icon} menu-panel-icon" }
                        span { class: "menu-panel-title", "{props.item.label}" }
                    }
                }

                // Children (only shown when expanded)
                if is_expanded {
                    div { class: "menu-panel-body",
                        for child in props.item.children.iter() {
                            TreeNode {
                                item: child.clone(),
                                level: props.level + 1,
                                expanded_items: props.expanded_items,
                                selected_item: props.selected_item,
                                on_select: props.on_select.clone()
                            }
                        }
                    }
                }
            }
        }
    } else {
        // Leaf items (no children) - simple selectable items
        rsx! {
            div {
                class: if is_selected { "menu-panel-item selected" } else { "menu-panel-item" },
                onclick: handle_select,

                div { class: "menu-panel-item-bullet", "•" }
                i { class: "{props.item.icon} menu-panel-item-icon" }
                span { class: "menu-panel-item-label", "{props.item.label}" }
            }
        }
    }
}