// src/docs/introduction.rs

pub struct IntroductionContent;

impl IntroductionContent {
    pub fn overview() -> &'static str {
        r#"
# Overview

This framework provides a modern, reactive approach to building web applications.
Built with Rust and WebAssembly, it offers type-safe development with excellent performance.

## Core Concepts

- **Components**: Reusable UI building blocks
- **Signals**: Reactive state management
- **Hooks**: Composable logic for components
- **Props**: Data flow between components
        "#
    }

    pub fn features() -> Vec<&'static str> {
        vec![
            "Type-safe component system",
            "Reactive updates with minimal re-renders",
            "Built-in routing support",
            "CSS-in-Rust styling options",
            "Hot reloading for rapid development",
            "Cross-platform support (Web, Desktop, Mobile)",
        ]
    }

    pub fn requirements() -> &'static str {
        r#"
## System Requirements

- Rust 1.70 or higher
- Node.js 16+ (for asset bundling)
- Modern web browser with WASM support

## Development Tools

- dioxus-cli for project management
- rust-analyzer for IDE support
- Browser DevTools for debugging
        "#
    }
}

// ============================================================================

// src/docs/getting_started.rs

pub struct GettingStartedContent;

impl GettingStartedContent {
    pub fn installation() -> &'static str {
        r#"
# Installation

## Install Dioxus CLI

```bash
cargo install dioxus-cli
```

## Create New Project

```bash
dx create my-app
cd my-app
```

## Install Dependencies

```bash
cargo build
```

## Run Development Server

```bash
dx serve
```

Your app will be available at http://localhost:8080
        "#
    }

    pub fn setup() -> &'static str {
        r#"
# Project Setup

## Project Structure

```
my-app/
├── src/
│   ├── main.rs
│   └── components/
├── assets/
│   └── styles.css
├── Cargo.toml
└── Dioxus.toml
```

## Configuration

Edit `Dioxus.toml` to customize your build:

```toml
[application]
name = "my-app"
default_platform = "web"

[web.app]
title = "My App"
```
        "#
    }

    pub fn first_app() -> &'static str {
        r#"
# Your First App

## Basic Component

```rust
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div {
            h1 { "Counter: {count}" }
            button {
                onclick: move |_| count += 1,
                "Increment"
            }
        }
    }
}

fn main() {
    launch(App);
}
```

## Run Your App

```bash
dx serve
```

Visit http://localhost:8080 to see your app in action!
        "#
    }
}

// ============================================================================

// src/docs/advanced.rs

pub struct AdvancedContent;

impl AdvancedContent {
    pub fn routing() -> &'static str {
        r#"
# Advanced Routing

## Define Routes

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/users/:id")]
    User { id: i32 },
    #[route("/settings")]
    Settings {},
}
```

## Navigation

```rust
let nav = navigator();

// Navigate programmatically
nav.push(Route::User { id: 42 });

// Or use Link component
rsx! {
    Link { to: Route::Settings {}, "Go to Settings" }
}
```
        "#
    }

    pub fn state_management() -> &'static str {
        r#"
# State Management

## Global State

```rust
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

#[component]
fn Counter() -> Element {
    rsx! {
        div {
            "Count: {COUNT}"
            button { onclick: move |_| *COUNT.write() += 1, "+" }
        }
    }
}
```

## Context API

```rust
#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::default()));
    
    rsx! {
        Child {}
    }
}

#[component]
fn Child() -> Element {
    let state = use_context::<Signal<AppState>>();
    // Use state...
}
```
        "#
    }

    pub fn performance() -> &'static str {
        r#"
# Performance Optimization

## Memoization

```rust
let expensive_value = use_memo(move || {
    // Expensive calculation
    data.iter().sum()
});
```

## Lazy Loading

```rust
let data = use_resource(move || async move {
    fetch_data().await
});

match &*data.read() {
    Some(Ok(data)) => rsx! { /* render */ },
    Some(Err(e)) => rsx! { "Error: {e}" },
    None => rsx! { "Loading..." },
}
```

## Code Splitting

Use dynamic imports for route-based code splitting:

```rust
#[component]
fn LazyRoute() -> Element {
    let component = use_future(|| async {
        // Load component asynchronously
    });
    // Render when ready
}
```
        "#
    }
}