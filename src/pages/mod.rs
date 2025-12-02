// src/pages/mod.rs
mod home;
//mod gallery;
mod settings;
mod documentation;

// src/pages/mod.rs
pub mod component_gallery;
pub mod blocks_gallery;
pub mod templates_gallery;

pub use home::HomePage;
//pub use gallery::GalleryPage;
pub use settings::SettingsPage;
pub use documentation::{DocumentationPage, get_all_docs};