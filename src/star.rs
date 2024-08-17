//a Imports
use geo_nd::Vector;
use serde::{Deserialize, Serialize};

use crate::{Subcube, Vec3};

//a Star and StarSerialized
//tp StarSerialized
/// This is the representation when a [Star] is serialized.
///
/// To reduce the size of serialized files this is a tuple (and hence
/// field names are not output many times over)
///
/// To enable serde serialization, the trait `From<Star>` is
/// implemented for [StarSerialized]; this preserves the information
/// required to reload the star without capturing its unit vector, or
/// neighbors or subcube data.
///
/// To enable serde deserialization, the trait `From<StarSerailized>`
/// is implemented for [Star]; this will create the star record with
/// derived values for its unit vector, subcube etc, without storing
/// them in the serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct StarSerialized(
    /// Id (e.g. Hipparcos number
    usize,
    /// Right-ascension, declination in radians
    f64,
    f64,
    /// Distance in light years
    f32,
    /// Visual magnitude and color (B-V)
    f32,
    f32,
);

//tp Star
/// A description of a star, usually in a Catalog
///
/// This is optimized to fit within 64 bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "StarSerialized", into = "StarSerialized")]
pub struct Star {
    /// User-specified id that is used for reference, indexing and
    /// searching.
    ///
    /// This must be unique for each star in a catalog. It
    /// is provided as a 'usize' as this is commonly a number, and it
    /// provides for simple serialization and deserialization of the
    /// [Star].
    pub id: usize,

    /// The right ascension of the star in radians
    pub ra: f64,

    /// The declination of the star in radians
    pub de: f64,

    /// The approximate distance to the star in lightyears
    pub ly: f32,

    /// The apparent magnitude of the star
    pub mag: f32,

    /// The blue-violet value for the star (a means to provide some
    /// color, type, or temperature for the star)
    pub bv: f32,

    /// A unit vector in the direction (hence a vector on the unit
    /// sphere)
    pub vector: Vec3,

    /// The subcube that the star's positon on the unit sphere lies within
    pub subcube: Subcube,
}

//ip From<Star> for StarSerialized
impl From<Star> for StarSerialized {
    fn from(star: Star) -> StarSerialized {
        StarSerialized(star.id, star.ra, star.de, star.ly, star.mag, star.bv)
    }
}

//ip From<StarSerialized> for Star
impl From<StarSerialized> for Star {
    fn from(star: StarSerialized) -> Star {
        Star::new(star.0, star.1, star.2, star.3, star.4, star.5)
    }
}

//ip Star
impl Star {
    //fi vec_of_ra_de
    /// Calculate a unit vector from a right ascension and declination
    pub fn vec_of_ra_de(ra: f64, de: f64) -> Vec3 {
        let vx = ra.cos() * de.cos();
        let vy = ra.sin() * de.cos();
        let vz = de.sin();
        [vx, vy, vz].into()
    }

    //ap brighter_than
    /// Return true if the magnitude is less than a value
    pub fn brighter_than(&self, mag: f32) -> bool {
        self.mag < mag
    }

    //cp new
    /// Create a new [Star] given its details
    pub fn new(id: usize, ra: f64, de: f64, ly: f32, mag: f32, bv: f32) -> Self {
        let vector = Self::vec_of_ra_de(ra, de);
        let subcube = Subcube::of_vector(&vector);
        Self {
            id,
            ra,
            de,
            ly,
            mag,
            bv,
            vector,
            subcube,
        }
    }

    //mp cos_angle_to
    /// Get the cosine of the angle between this [Star] and another
    pub fn cos_angle_between(&self, other: &Star) -> f64 {
        self.vector.dot(&other.vector)
    }
}
