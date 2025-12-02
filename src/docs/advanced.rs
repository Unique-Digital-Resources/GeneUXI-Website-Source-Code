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