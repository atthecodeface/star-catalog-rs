//!
//! # Star catalog library
//!
//! This library provides types that describe a star, and catalogs of
//! stars. The data maintained for a star is relatively simple: its
//! position with respect to the sun, approximate magnitude and color.
//!
//! The purpose of the library is to allow simple applications such as
//! star databases, constellation viewers, or sky map generation. The aim
//! is not to support a full-blown astronomical dataset.
//!
//! # Included star catalogs
//!
//! The library includes the complete set of Hipparcos stars that have all
//! the required information (116,812 stars). (The original Hipparcos
//! catalog has a total of 118,218 stars, but some do not have parallax or
//! visual magnitude values).
//!
//! There is no standard naming for stars; however, the IAU has some
//! standard names, and the [iau] module includes the naming from
//! <https://www.iau.org/public/themes/naming_stars> - for those
//! approved up to Jan 2021. This database includes the IAU right
//! ascension and declination, which is used to find the closest star
//! in the Hipparcos database. Note that some IAU named stars are
//! *not* in the Hipparcos database.
//!
//! If the `hipp_bright` feature is used then the Hipparcos catalog
//! stars of magnitude 8.0 or brighter are included (41,013 stars) as
//! a postcard string, as [hipparcos::HIPP_BRIGHT_PST]; also 430 'common'
//! names of these stars are included as
//! [hipparcos::HIP_COLLATED_ALIASES].
//!
//! # [Catalog], [Star], cube and [Subcube]
//!
//! Stars themselves are described by right ascension and declination;
//! the [Star] type keeps these in radians. Distance to a star is kept
//! in light years; it is likely to be known to a relatively poor
//! accuracy compared to the position of the star in the sky.
//!
//! A star also contains an 'id'; this is a usize whose value is up to
//! the catalog or user. For the Hipparcos catalog this is the
//! Hipparcos catalog number; it is provided as a usize, rather than a
//! generic, to provide for simple serialization of catalogs. The id
//! of each star must be unique within a catalog (so it can be used
//! for identifying a specific star).
//!
//! Each star maintains a unit vector in its direction, which can be
//! viewed as placing the star on the unit sphere centered on the
//! origin.
//!
//! A catalog is a collection of stars, with optional names; its
//! supports indexing and searching by id or name, and by geometry.
//!
//! A catalog can only be searched once it has been fully populated;
//! at this point the internal structures can be set up (and the stars
//! sorted, for example, by id).
//!
//! To enable efficient searching by geometry the cube centred on the
//! origin with corners at +-1 is subdivided in each dimension; this
//! yields the [Subcube]. Each star's vector is then within one of the
//! subcubes. The [Catalog] maintains list of stars within each
//! subcube, but only once the catalog has been fully populated. The
//! subcubes can then be used for the efficient geographical
//! searching.
//!
//! # Precision
//!
//! The naked eye has a resolution of the order of 1 arcminute; this
//! resolution easily fits in a single precision (f32) value for
//! angles in radians.
//!
//! For photographic images using standard lenses the field of view
//! will be, at smmallest, in the order of 1 degrees. With modern
//! cameras at 10,000 pixels across a sensor frame a single pixel is
//! about 0.3 arcseconds; storing angles in f32 in radians just about
//! suffices for this.
//!
//! For telescopic images, though, the precision required to specify
//! the difference between two pixels in an image that could be from
//! any part of the sky will exceed that provided by f32 (which is
//! only roughly one part in 8E6).
//!
//! Hence the [Star] precision for position must be held as f64 values.
//!
//! # Fundamental types
//!
//! Stars maintain their right ascension and declination as f64
//! values, but the less precise magnitude, distance, etc can be held
//! as f32.
//!
//! Given the star's position is held as f64, the unit vector for the
//! star (which the [Vec3] type provides) is held with f64
//! components.
//!
//! # How to use
//!
//! First, create a catalog. Usually a catalog is loaded from a file:
//!
//! ```rust,ignore
//!    let s = std::fs::read_to_string("hipparcos.json")?;
//!    let mut catalog: Catalog = serde_json::from_str(&s)?;
//! ```
//!
//! Before adding names the catalog must be sorted; the names will
//! refer to *sorted* entries in the catalog (this restriction will be
//! removed in the future, just not yet)
//!
//! ```rust,ignore
//!    catalog.sort();
//! ```
//!
//! Stars in the catalog can then be named; this applies names to the *id*s of {Star]s in the catalog.
//!
//! (The aliases in hipparcos::HIP_ALIASES are somewhat developer-specified...)
//!
//! ```rust,ignore
//!    catalog.add_names(hipparcos::HIP_ALIASES)?;
//! ```
//!
//! Before performing geometric searching
//!
//! ```rust,ignore
//!    catalog.derive_data();
//! ```
//!
//! Find a star by id
//!
//! ```rust,ignore
//!   let polaris : CatalogIndex = catalog.find_sorted(11767).expect("Should have found Polaris");
//! ```
//!
//! Find a star by name
//!
//! ```rust,ignore
//!   let polaris_by_name = catalog.find_name("Polaris").expect("Should have found Polaris");
//!   assert_eq!(catalog[polaris_by_name].id(), 111767);
//! ```
//!
//! Find a star closest to a right-ascension and declination (and
//! return the cosine of the angle offset):
//!
//! ```rust,ignore
//!   let (_,polaris_by_ra_de) = catalog.closest_to(0.66, 1.555).expect("Should have found Polaris");
//!   assert_eq!(catalog[polaris_by_ra_de].id(), 111767);
//! ```
//!
//! Find possible sets of three stars (A, B C) where the three angles between A and B, A and C, and B and C are given - to within an angular tolerance of delta:
//!
//! ```rust,ignore
//!   let candidate_tris = catalog.find_star_triangles(catalog.iter_all(), &[0.1, 0.15, 0.05], 0.003);
//! ```
//!
//! # A full-blown example
//!
//! ```rust
//!    use star_catalog::{hipparcos, Catalog, CatalogIndex};
//!
//! # fn main() -> Result<(),Box<dyn std::error::Error>> {
//!    let s = std::fs::read_to_string("hipparcos.json")?;
//!    let mut catalog: Catalog = serde_json::from_str(&s)?;
//!    catalog.sort();
//!    catalog.add_names(hipparcos::HIP_ALIASES, true)?;
//!    catalog.derive_data();
//!    let polaris : CatalogIndex = catalog.find_sorted(11767).expect("Should have found Polaris");
//!    let polaris_by_name = catalog.find_name("Polaris").expect("Should have found Polaris");
//!    assert_eq!(catalog[polaris_by_name].id, 11767);
//!    let (_,polaris_by_ra_de) = catalog.closest_to(0.66, 1.555).expect("Should have found Polaris");
//!    assert_eq!(catalog[polaris_by_ra_de].id, 11767);
//! # Ok(())
//! # }
//! ```
//! # Crate Feature Flags
//!
//! The following crate feature flags are available. They are configured in your Cargo.toml.
//!
//! * csv
//!
//!    * Optional, compatible with Rust stable
//!
//!    * Allows reading of CSV catalog files (such as hipparcos::read_to_catalog)
//!
//! * image
//!
//!    * Optional, compatible with Rust stable
//!
//!    * Module to provide means to create images, and to add skymap
//!      images and cubemap to star_catalog binary
//!
//! * postcard
//!
//!    * Optional, compatible with Rust stable
//!    * Allows reading and writing catalog files in Postcard format
//!
//! * hipp_bright
//!
//!    * Optional, compatible with Rust stable
//!    * Includes constants for the Hipparcos catalog
//!

/// An XY vector used for star positions within an image
pub type Vec2 = geo_nd::FArray<f64, 2>;

/// An XYZ vector used for star directions
pub type Vec3 = geo_nd::FArray<f64, 3>;

/// A vector required by the Quat type - otherwise unused in the star catalog
pub type Vec4 = geo_nd::FArray<f64, 4>;

/// A quaternion that represents orientations of views of a sky map;
/// this includes the direction and 'up' for a camera, for example
pub type Quat = geo_nd::QArray<f64, Vec3, Vec4>;

mod catalog;
mod error;
mod star;
mod subcube;

pub mod cmdline;
pub mod constellations;
pub mod hipparcos;
pub mod iau;

#[cfg(feature = "image")]
mod image;

pub use catalog::{Catalog, CatalogIndex};
pub use error::Error;
pub use star::Star;
pub use subcube::Subcube;

#[cfg(feature = "image")]
pub use image::{ImageView, StarDrawStyle};
