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
//! https://www.iau.org/public/themes/naming_stars - for those
//! approved up to Jan 2021. This database includes the IAU right
//! ascension and declination, which is used tto find the closest star
//! in the Hipparcos database. Note thtata some IAU named stars are
//! *not* in the Hipparcos database.
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
//! Each star maintains a
//! unit vector in its direction, which can be viewed as placing the
//! star on the unit sphere centered on the origin.
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

pub type Vec3 = geo_nd::FArray<f32, 3>;
pub type Vec4 = geo_nd::FArray<f32, 4>;
pub type Quat = geo_nd::QArray<f32, Vec3, Vec4>;

mod subcube;
pub use subcube::Subcube;

mod star;
pub use star::Star;

mod catalog;
pub mod hipparcos;
pub mod iau;
pub use catalog::Catalog;
