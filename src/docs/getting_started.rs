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