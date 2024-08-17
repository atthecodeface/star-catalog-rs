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
    f32,
    f32,
    /// Distance in light years
    f32,
    /// Visual magnitude and color (B-V)
    f32,
    f32,
);

//tp Star
/// A star record (which is roughly 64 bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "StarSerialized", into = "StarSerialized")]
pub struct Star {
    id: usize,
    ra: f32,
    de: f32,
    ly: f32,
    vmag: f32,
    bv: f32,
    vector: Vec3,
    subcube: Subcube,
    neighbors: Vec<(f32, usize)>,
}

//ip From<Star> for StarSerialized
impl From<Star> for StarSerialized {
    fn from(star: Star) -> StarSerialized {
        StarSerialized(star.id, star.ra, star.de, star.ly, star.vmag, star.bv)
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
    pub fn vec_of_ra_de(ra: f32, de: f32) -> Vec3 {
        let vx = ra.cos() * de.cos();
        let vy = ra.sin() * de.cos();
        let vz = de.sin();
        [vx, vy, vz].into()
    }

    //ap subcube
    /// Return the subcube the [Star] is in
    pub fn subcube(&self) -> Subcube {
        self.subcube
    }

    //ap id
    /// Get the id of the [Star]
    pub fn id(&self) -> usize {
        self.id
    }

    //ap mag
    /// Get the magnitude of the [Star]
    pub fn mag(&self) -> f32 {
        self.vmag
    }

    //ap vector
    /// Get the unit vector of the [Star]
    pub fn vector(&self) -> &Vec3 {
        &self.vector
    }

    //cp new
    /// Create a new [Star] given its details
    pub fn new(id: usize, ra: f32, de: f32, ly: f32, vmag: f32, bv: f32) -> Self {
        let neighbors = vec![];
        let vector = Self::vec_of_ra_de(ra, de);
        let subcube = Subcube::of_vector(&vector);
        Self {
            id,
            ra,
            de,
            ly,
            vmag,
            bv,
            vector,
            subcube,
            neighbors,
        }
    }

    //mp cos_angle_to
    /// Get the cosine of the angle between this [Star] and another
    pub fn cos_angle_between(&self, other: &Star) -> f32 {
        self.vector.dot(&other.vector)
    }

    //mp clear_neighbors
    /// Clear the neighbors; this must be invoked if the stars are
    /// renumbered in a Catalog
    pub fn clear_neighbors(&mut self) {
        self.neighbors.clear();
    }

    //mp add_neighbor
    /// Add a neighbor (index) to the list for this star, with the cosine of
    /// the angle between them
    pub fn add_neighbor(&mut self, cos_angle: f32, other: usize) {
        self.neighbors.push((cos_angle, other));
    }
}
