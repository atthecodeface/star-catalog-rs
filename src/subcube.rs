
use geo_nd::Vector;

use crate::Vec3;

//tp Subcube
/// A representation of a portion of the unit cube, to improve
/// geometric searching of a catalog.
///
/// In a catalog stars can be marked as being on a part of the unit
/// sphere. A subcube is created by dividing the cube from (-1, 1) in
/// each dimension into ELE_PER_SIDE^3 subcubes;
/// subcubes are easy to manipulate, and can be related to the portion
/// of the unit sphere they encompass.
///
/// Not all subcubes include a portion of the unit sphere; in a
/// catalog this would indicate there will be an empty list for that
/// subcube. The catalog can maintain either a HashMap keyed on
/// Subcube or a Vec indexed by subcube, of lists of stars within that
/// subcube.
///
/// A subcube has up to 8 immediate neighbors (some may be out of
/// bounds).
///
/// The [Subcube] provides methods for iterating over it and its
/// neighbors, and within a region around the subcube.
///
/// # Rationale
///
/// Subdividing the unit sphere can be handled in many ways; the
/// subcube division provides for simple iteration over space and over
/// stars that are clustered together, which is a common search
/// problem.
///
/// # Choice of ELE_PER_SIDE
///
/// If ELE_PER_SIDE were 2 then the maximum angle of difference
/// between any two points in a subcube would be 90 degrees
///
/// If ELE_PER_SIDE were 4 the the maxim angle of difference between
/// any two points in a subcube would be 45 degrees
///
/// If ELE_PER_SIDE were 32 then each subcube has side length 1/16; a
/// maximum angle subtended of the line betwen two opposite corners
/// and the centre of the circle as 2.asin(sqrt(3)*r/16 / r / 2) =
/// 2.asin(sqrt(3)/32) = 6 degrees
///
/// Here also there is a shortest distance (minimum) of r/16 from
/// corner of a cube, across its neighbor, to a the next neighbor (it
/// will be more than that, but this is a mininum); this subtends an
/// angle of 2*asin(1/16/2), or 3.58 degrees.  cos(this) is
/// 0.99804. Hence a star within a subcube must be at least 3.58
/// degrees from any star that is not within the subcube *or* one of
/// its immediate neighbors - or rather, all stars within 3.58 degrees
/// of the star MUST be within the same subcube as the star, or in one
/// of its immediate neighbors
///
/// If ELE_PER_SIDE were 64 then each subcube has side length 1/32; a
/// maximum angle subtended of the line betwen two opposite corners
/// and the centre of the circle as 2.asin(sqrt(3)*r/32 / r / 2) =
/// 2.asin(sqrt(3)/64) = 3 degrees
///
/// If ELE_PER_SIDE were 16 then theshortest distance (minimum) is
/// r/8 from corner of a cube, across its neighbor, to a
/// non-neighbor (it will be more than that, but this is a mininum);
/// this subtends an angle of 2*asin(1/8/2), or 7.17 degrees
/// cos(this) is 0.99219.
///
/// Emmpirically, using the Hipparcos star catalog:
///
/// ELE_PER_SIDE of 32 is needed for cos() > 0.9980 (3.62 degrees);
/// 276,038 (/2) pairs of vmag<7.0
///
/// ELE_PER_SIDE of 16 is needed for cos() > 0.995 (5.73 degrees)
/// There are 679082 (/2) pairs of stars of vmag < 7.0 separated by at most that
///
/// ELE_PER_SIDE of 8 is needed for cos() > 0.992 (7.25 degrees)
/// There are 1080842 (/2) pairs of stars of vmag < 7.0 separated by at most that
///
/// ## Angles of subcubes
///
/// With 32 subcubes the subcube_max_angle is 6.18 degrees
///
/// * 2 subcubes = 12.4 degrees
/// * 3 subcubes = 18.6 degrees
/// * 4 subcubes = 24.8 degrees
/// * 5 subcubes = 30.9 degrees
/// * 6 subcubes = 37.1 degrees
/// * 7 subcubes = 43.3 degrees
/// * 8 subcubes = 49.5 degrees
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Subcube(u32);

//ip Subcube
impl Subcube {
    const HALF_ELE_PER_SIDE: usize = 16;
    //cp ELE_PER_SIDE
    /// The number of subdivisions per dimension of the (-1,1) cube
    /// that produces the subcubes
    pub const ELE_PER_SIDE: usize = Self::HALF_ELE_PER_SIDE * 2;

    const ELE_PER_SIDE2: usize = Self::ELE_PER_SIDE * Self::ELE_PER_SIDE;

    /// The number of subcubes in the (-1,1) cube
    ///
    /// A Catalog may have a Vec with this number of entries; each
    /// element would be a Vec, but only those that contain stars (and
    /// are therefore on the unit sphere) will contain elements, the
    /// rest would be empty
    ///
    /// It is worth noting that as ELE_PER_SIDE increases, the number
    /// of populated subcubes approaches 2*ELE_PER_SIDE^2
    pub const NUM_SUBCUBES: usize = Self::ELE_PER_SIDE * Self::ELE_PER_SIDE * Self::ELE_PER_SIDE;

    /// The size of each side of a Subcube
    pub const SUBCUBE_SIZE: f64 = 1.0 / Self::HALF_ELE_PER_SIDE as f64;

    /// The raduis of the circumsphere of a Subcube - i.e. all stars
    /// within the subcube must be closer tthan this to the centre of
    /// the Subcube (although some such stars may be in a neighboring
    /// subcube)
    pub const SUBCUBE_RADIUS: f64 = 1.7321 * (Self::SUBCUBE_SIZE / 2.0);

    //fi index_of_coord
    /// Get the subcube index of a coordinate
    ///
    /// The coordinate must be in the range -1. to 1.
    fn index_of_coord(c: f64) -> usize {
        let c = c.min(1.);
        ((c + 1.0).abs() * (Self::HALF_ELE_PER_SIDE as f64) * 0.999_999).floor() as usize
    }

    //fi coord_of_index
    /// Get the coordinate of the centre of an index
    fn coord_of_index(i: usize) -> f64 {
        (2 * i + 1) as f64 / Self::ELE_PER_SIDE as f64 - 1.0
    }

    //cp of_vector
    /// Get the subcube of a unit vector (which is thus a point on the
    /// unit sphere)
    pub fn of_vector(vector: &[f64; 3]) -> Self {
        let xe = Self::index_of_coord(vector[0]);
        let ye = Self::index_of_coord(vector[1]);
        let ze = Self::index_of_coord(vector[2]);
        (xe, ye, ze).into()
    }

    //ap as_usize
    /// Get a value for the subcube, different for each within the (-1,1) cube
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    //ap center_non_unit
    /// Get the vector of the centre of the Subcube (this is *NOT* a unit vector)!
    pub(crate) fn center_non_unit(&self) -> Vec3 {
        let (x, y, z): (usize, usize, usize) = self.into();
        [
            Self::coord_of_index(x),
            Self::coord_of_index(y),
            Self::coord_of_index(z),
        ]
        .into()
    }

    //ap center
    /// Get the vector of the centre of the Subcube (this is *NOT* a unit vector)!
    pub fn center(&self) -> [f64; 3] {
        self.center_non_unit().into()
    }

    //ap may_be_on_sphere
    /// Return true if the subcube might contain part of the unit
    /// sphere
    ///
    /// This returns false if the centre is too far from the unit
    /// sphere for any part of the subcube to overlap the unit sphere
    pub fn may_be_on_sphere(&self) -> bool {
        let r = self.center_non_unit().length();
        (1.0 - Self::SUBCUBE_RADIUS..=1.0 + Self::SUBCUBE_RADIUS).contains(&r)
    }

    /// Return true if the subcube vector is within 'cos' of the vector v
    ///
    /// v should be a unit vector (i.e. on the unit sphere)
    pub(crate) fn within_cos_of_vector(&self, v: &Vec3, cos: f64) -> bool {
        let c = self.center_non_unit();
        let r = c.length();
        v.dot(&c) / r >= cos
    }

    /// Return None if the subcube definitely does no intersect with the unit sphere.
    ///
    /// Returns Some(cos(angle)) if the subcube might intersect with
    /// the unit sphere, where 'angle' is the angle between the
    /// suppliid vector and the centre of the Subcube. If searching
    /// for all Subcubes that could contain stars that are within a
    /// certain angle of a particular unit vector then use this
    /// function and compare the cos it returns (if any) with the cos
    /// of (angle of search + SUBCUBE ANGLE)
    ///
    /// v *MUST* be a unit vector (i.e. on the unit sphere)
    pub(crate) fn cos_angle_on_sphere(&self, v: &Vec3) -> Option<f64> {
        let c = self.center_non_unit();
        let r = c.length();
        if (1.0 - Self::SUBCUBE_RADIUS..=1.0 + Self::SUBCUBE_RADIUS).contains(&r) {
            Some(v.dot(&c) / r)
        } else {
            None
        }
    }

    /// Get an iterator over all the Subcubes in the (-1, 1) cube
    pub fn iter_all() -> impl Iterator<Item = Subcube> + Clone {
        SubcubeRangeIter {
            xyz: (0, 0, 0),
            xrange: 0..Self::ELE_PER_SIDE,
            yrange: 0..Self::ELE_PER_SIDE,
            zrange: 0..Self::ELE_PER_SIDE,
        }
    }

    /// Get an iterator over all the Subcubes that may be on the sphere
    pub fn iter_sphere() -> impl Iterator<Item = Subcube> + Clone {
        SubcubeSphereIter::default()
    }

    /// Get an iterator over all the Subcubes that may be on the sphere whose cos(angle) to a vector is at least a given value
    pub fn iter_sphere_within_cos_of_vector(
        v: &[f64; 3],
        cos: f64,
    ) -> impl Iterator<Item = Subcube> + Clone {
        let v: Vec3 = v.into();
        SubcubeSphereIter::default().filter(move |s| s.within_cos_of_vector(&v, cos))
    }

    //mp iter_range
    /// Get an iterator over this Subcube and all with X, Y or Z
    /// coordinates within dxyz of it
    ///
    /// A value of 0 for dxyz returns an iterator over just this one
    /// Subcube
    ///
    /// A value of 1 for dxyz returns an iterator over this subcube
    /// and all its immediate neighbors
    pub fn iter_range(&self, dxyz: usize) -> impl Iterator<Item = Subcube> + Clone + 'static {
        let xyz: (usize, usize, usize) = (*self).into();
        let xmin = xyz.0.saturating_sub(dxyz);
        let xmax = (xyz.0 + dxyz + 1).min(Self::ELE_PER_SIDE);
        let ymin = xyz.1.saturating_sub(dxyz);
        let ymax = (xyz.1 + dxyz + 1).min(Self::ELE_PER_SIDE);
        let zmin = xyz.2.saturating_sub(dxyz);
        let zmax = (xyz.2 + dxyz + 1).min(Self::ELE_PER_SIDE);
        SubcubeRangeIter {
            xyz: (xmin, ymin, zmin),
            xrange: xmin..xmax,
            yrange: ymin..ymax,
            zrange: zmin..zmax,
        }
    }
}

//ip From<Subcube> for (usize, usize, usize)
impl From<Subcube> for (usize, usize, usize) {
    fn from(s: Subcube) -> (usize, usize, usize) {
        (&s).into()
    }
}

//ip From<&Subcube> for (usize, usize, usize)
impl From<&Subcube> for (usize, usize, usize) {
    fn from(s: &Subcube) -> (usize, usize, usize) {
        let s = s.as_usize();
        let x = s % Subcube::ELE_PER_SIDE;
        let y = (s / Subcube::ELE_PER_SIDE) % Subcube::ELE_PER_SIDE;
        let z = s / Subcube::ELE_PER_SIDE2;
        (x, y, z)
    }
}

//ip From<(usize, usize, usize)> for Subcube
impl From<(usize, usize, usize)> for Subcube {
    fn from((x, y, z): (usize, usize, usize)) -> Subcube {
        let s = x + y * Self::ELE_PER_SIDE + z * Self::ELE_PER_SIDE2;
        Subcube(s as u32)
    }
}

//ip std::ops::Add<isize> for Subcube
impl std::ops::Add<isize> for Subcube {
    type Output = Subcube;
    fn add(self, delta: isize) -> Subcube {
        let s = self.0 as isize + delta;
        assert!(
            s >= 0,
            "Delta of Subcube used to take subcube out of bounds"
        );
        Subcube(s as u32)
    }
}

/// Iterator over a range of Subcubes
#[derive(Debug, Clone)]
pub struct SubcubeRangeIter {
    xyz: (usize, usize, usize),
    xrange: std::ops::Range<usize>,
    yrange: std::ops::Range<usize>,
    zrange: std::ops::Range<usize>,
}

impl Iterator for SubcubeRangeIter {
    type Item = Subcube;
    fn next(&mut self) -> Option<Subcube> {
        if !self.xrange.contains(&self.xyz.0) {
            self.xyz.0 = self.xrange.start;
            self.xyz.1 += 1;
        }
        if !self.yrange.contains(&self.xyz.1) {
            self.xyz.0 = self.xrange.start;
            self.xyz.1 = self.yrange.start;
            self.xyz.2 += 1;
        }
        if !self.zrange.contains(&self.xyz.2) {
            return None;
        }
        let subcube = self.xyz.into();
        self.xyz.0 += 1;
        Some(subcube)
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum SubcubeSphereIterState {
    // Starting - set z to 0, and then begin
    #[default]
    Init,
    // Start a 'z' layer, at the current value of 'z'
    StartZ,
    // Start a 'y' row, at the current value of 'z' and 'y'
    StartY,
    // Finished a 'z' layer; increment z and if not finished completely, StartZ
    FinishedZ,
    // Start a row (row_start_x valid, x=row_start_x, y=0, z=0;)
    //
    // Return Some(+-x,+-y,+-z), move to another state, or decrement x, or if x is already 0 then FinishedZ
    //
    // bitmask goes 0 through 7; bit 0 indicates use minus X, bit 1 indicates use minus Y, bit 2 indicates use minus Z (where minus is the other side of half-the-subcubes)
    YieldXYZ,
}
/// Iterator over the subcubes that include/touch the unit sphere
#[derive(Debug, Clone, Default)]
pub struct SubcubeSphereIter {
    state: SubcubeSphereIterState,
    z: usize,
    row_start_x: usize,
    y: usize,
    x: usize,
    bitmask: u8,
}

impl Iterator for SubcubeSphereIter {
    type Item = Subcube;
    fn next(&mut self) -> Option<Subcube> {
        loop {
            match self.state {
                SubcubeSphereIterState::Init => {
                    self.z = 0;
                    self.state = SubcubeSphereIterState::StartZ;
                }
                SubcubeSphereIterState::StartZ => {
                    // Start at the 'ceiling' x for the row
                    let z = (self.z as f64) * Subcube::SUBCUBE_SIZE;
                    self.row_start_x = (Subcube::index_of_coord((1.0 - z * z).sqrt())
                        - Subcube::HALF_ELE_PER_SIDE
                        + 1)
                    .min(Subcube::HALF_ELE_PER_SIDE - 1);
                    // eprintln!("Start layer Z {}", self.z);
                    self.y = 0;
                    self.state = SubcubeSphereIterState::StartY;
                }
                SubcubeSphereIterState::StartY => {
                    // Start at the given row start (possibly from previous row)
                    self.x = self.row_start_x;
                    // eprintln!("Start row Y {} {} {}", self.z, self.y, self.x);
                    self.bitmask = 0;
                    let mut subcube: Subcube = (
                        Subcube::HALF_ELE_PER_SIDE + self.x,
                        Subcube::HALF_ELE_PER_SIDE + self.y,
                        Subcube::HALF_ELE_PER_SIDE + self.z,
                    )
                        .into();
                    loop {
                        if subcube.may_be_on_sphere() {
                            // Start *next* row at this row_start (as the start of increasing Y row is always decreasing X values)
                            self.row_start_x = self.x;
                            // Yield all 8 of +-xyz, using the bitmask
                            self.bitmask = 0;
                            self.state = SubcubeSphereIterState::YieldXYZ;
                            break;
                        }
                        if self.x == 0 {
                            // For a given z, y can have been incremented such that all z^2 + y^2 > 1, and hence all x are outside the sphere; end the Z layer
                            self.state = SubcubeSphereIterState::FinishedZ;
                            break;
                        }
                        subcube = subcube + (-1);
                        self.x -= 1;
                    }
                }
                SubcubeSphereIterState::YieldXYZ => {
                    let mut xyz = (
                        Subcube::HALF_ELE_PER_SIDE + self.x,
                        Subcube::HALF_ELE_PER_SIDE + self.y,
                        Subcube::HALF_ELE_PER_SIDE + self.z,
                    );
                    if (self.bitmask & 1) != 0 {
                        xyz.0 = Subcube::HALF_ELE_PER_SIDE - 1 - self.x;
                    }
                    if (self.bitmask & 2) != 0 {
                        xyz.1 = Subcube::HALF_ELE_PER_SIDE - 1 - self.y;
                    }
                    if (self.bitmask & 4) != 0 {
                        xyz.2 = Subcube::HALF_ELE_PER_SIDE - 1 - self.z;
                    }
                    self.bitmask += 1;
                    if self.bitmask >= 8 {
                        let mut finished_row = self.x == 0;
                        if !finished_row {
                            self.x -= 1;
                            let subcube: Subcube = (
                                Subcube::HALF_ELE_PER_SIDE + self.x,
                                Subcube::HALF_ELE_PER_SIDE + self.y,
                                Subcube::HALF_ELE_PER_SIDE + self.z,
                            )
                                .into();
                            finished_row = !subcube.may_be_on_sphere();
                        }
                        if finished_row {
                            self.y += 1;
                            if self.y >= Subcube::HALF_ELE_PER_SIDE {
                                self.state = SubcubeSphereIterState::FinishedZ;
                            } else {
                                self.state = SubcubeSphereIterState::StartY;
                            }
                        } else {
                            self.bitmask = 0;
                        }
                    }
                    return Some(xyz.into());
                }
                SubcubeSphereIterState::FinishedZ => {
                    self.z += 1;
                    if self.z >= Subcube::HALF_ELE_PER_SIDE {
                        return None;
                    } else {
                        self.state = SubcubeSphereIterState::StartZ;
                    }
                }
            }
        }
    }
}
