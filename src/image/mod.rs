//! # Image creation libray (requires `image` feature)
/// This library utilizes the [image] crate to provide the ability to
/// create images of sky maps
mod image_view;
pub use image_view::{ImageView, StarDrawStyle};
