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
///
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
    //ap temp
    /// Get an temperature for the star
    pub fn temp(&self) -> f32 {
        let t = 4600.0 * (1.0 / (1.7 + 0.92 * self.bv) + 1.0 / (0.62 + 0.92 * self.bv));
        // eprintln!("{} {}", self.bv, t);
        t
    }

    //fp temp_to_rgb
    /// This only really works for t >= 1600
    ///
    /// The first stage is to convert to black body CIE XY
    /// coordinates; then to convert to linear RGB, and finally to
    /// sRGB (which provides gamma correction for 'standard' RGB that
    /// modern OSes use)
    pub fn temp_to_rgb(t: f32) -> (f32, f32, f32) {
        let x = {
            if t <= 4000.0 {
                (-0.2661239E9 / t.powi(3))
                    + (-0.2343580E6 / t.powi(2))
                    + (0.8776956E3 / t)
                    + 0.179910
            } else {
                (-3.0258469E9 / t.powi(3))
                    + (2.1070379E6 / t.powi(2))
                    + (0.2226347E3 / t)
                    + 0.240390
            }
        };

        let y = {
            if t <= 2222.0 {
                -1.1063814 * x.powi(3) - 1.34811020 * x.powi(2) + 2.18555832 * x - 0.20219683
            } else if t > 2222.0 && t <= 4000.0 {
                -0.9549476 * x.powi(3) - 1.37418593 * x.powi(2) + 2.09137015 * x - 0.16748867
            } else {
                3.0817580 * x.powi(3) - 5.87338670 * x.powi(2) + 3.75112997 * x - 0.37001483
            }
        };

        let cie_y = 1.0;
        let (cie_x, cie_z) = {
            if y <= 0.0 {
                (0.0, 0.0)
            } else {
                ((x * cie_y) / y, ((1.0 - x - y) * cie_y) / y)
            }
        };

        fn linear_to_srgb(c: f32) -> f32 {
            if c < 0.0031308 {
                c * 12.92
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            }
        }
        let r = 3.2406 * cie_x - 1.5372 * cie_y - 0.4986 * cie_z;
        let g = -0.9689 * cie_x + 1.8758 * cie_y + 0.0415 * cie_z;
        let b = 0.0557 * cie_x - 0.2040 * cie_y + 1.0570 * cie_z;

        (linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(b))
    }

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
