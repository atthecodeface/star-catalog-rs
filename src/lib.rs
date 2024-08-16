/*!
Documentation

 */

pub type Vec3 = geo_nd::FArray<f32, 3>;
pub type Vec4 = geo_nd::FArray<f32, 4>;
pub type Quat = geo_nd::QArray<f32, Vec3, Vec4>;

mod subcube;
pub use subcube::{Subcube, SubcubeMask};

mod star;
pub use star::Star;

mod catalog;
pub mod hipparcos;
pub mod iau;
pub use catalog::Catalog;
