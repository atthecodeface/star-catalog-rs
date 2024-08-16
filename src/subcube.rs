use crate::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Subcube(usize);

#[derive(Debug, Clone, Copy)]
pub struct SubcubeMask(usize);
impl SubcubeMask {
    const ALL: SubcubeMask = SubcubeMask(0);

    const MASK_XL: SubcubeMask = SubcubeMask(0x1249249);
    const MASK_XR: SubcubeMask = SubcubeMask(0x4924924);

    const MASK_YL: SubcubeMask = SubcubeMask(0x01c0e07);
    const MASK_YR: SubcubeMask = SubcubeMask(0x70381c0);

    const MASK_ZL: SubcubeMask = SubcubeMask(0x00001ff);
    const MASK_ZR: SubcubeMask = SubcubeMask(0x7fc0000);
}
impl std::ops::BitOr for SubcubeMask {
    type Output = SubcubeMask;
    fn bitor(self, other: SubcubeMask) -> Self {
        Self(self.0 | other.0)
    }
}
impl SubcubeMask {
    fn disable(&self, index: usize) -> bool {
        (self.0 >> index) & 1 == 1
    }
}

impl Subcube {
    /// An oct-tree enclosing a sphere of radius 1.0 - eps with 2^3 elements
    /// has a maximum angle of difference between any two points in a
    /// section of 90 degrees
    ///
    /// An oct-tree enclosing a sphere of radius 1.0 - eps with 4^3 elements
    /// has a maximum angle of difference between any two points in a
    /// section of 45 degrees
    ///
    /// An oct-tree enclosing a sphere of radius 1.0 - eps with 32^3
    /// elements has each element of side length r/16; a maximum angle
    /// subtended of the line betwen two opposite corners and the centre of
    /// the circle as 2.asin(sqrt(3)*r/16 / r / 2) = 2.asin(sqrt(3)/32) = 6
    /// degrees
    ///
    /// An oct-tree enclosing a sphere of radius 1.0 - eps with 64^3
    /// elements has each element of side length r/32; a maximum angle
    /// subtended of the line betwen two opposite corners and the centre of
    /// the circle as 2.asin(sqrt(3)*r/32 / r / 2) = 2.asin(sqrt(3)/32) = 3
    /// degrees
    ///
    /// dividing into 32x32x32 cubes has shortest distance (minimum) of
    /// r/16 from corner of a cube, across its neighbor, to a
    /// non-neighbor (it will be more than that, but this is a mininum);
    /// this subtends an angle of 2*asin(1/16/2), or 3.58 degrees
    /// cos(this) is 0.99804
    ///
    /// dividing into 16x16x16 cubes has shortest distance (minimum) of
    /// r/8 from corner of a cube, across its neighbor, to a
    /// non-neighbor (it will be more than that, but this is a mininum);
    /// this subtends an angle of 2*asin(1/8/2), or 7.17 degrees
    /// cos(this) is 0.99219
    ///
    /// Emmpirically:
    ///
    /// 32 is needed for cos() > 0.9980 (3.62 degrees); 276,038 (/2) pairs of vmag<7.0
    ///
    /// 16 is needed for cos() > 0.995 (5.73 degrees)
    /// There are 679082 (/2) pairs of stars of vmag < 7.0 separated by at most that
    ///
    /// 16 is needed for cos() > 0.9921 (7.21 degrees)
    /// There are 1067374 (/2) pairs of stars of vmag < 7.0 separated by at most that
    ///
    /// 8 is needed for cos() > 0.992 (7.25 degrees)
    /// There are 1080842 (/2) pairs of stars of vmag < 7.0 separated by at most that
    const ELE_PER_SIDE: usize = 32;
    const ELE_PER_SIDE2: usize = Self::ELE_PER_SIDE * Self::ELE_PER_SIDE;
    pub const NUM_SUBCUBES: usize = Self::ELE_PER_SIDE * Self::ELE_PER_SIDE * Self::ELE_PER_SIDE;

    //fi delta
    const fn delta(b: usize) -> isize {
        let b = b as isize;
        let x = b % 3;
        let y = (b / 3) % 3;
        let z = (b / 9) % 3;
        (x - 1) + (y - 1) * (Self::ELE_PER_SIDE as isize) + (z - 1) * (Self::ELE_PER_SIDE2 as isize)
    }

    const DELTAS: [isize; 27] = [
        Self::delta(0),
        Self::delta(1),
        Self::delta(2),
        Self::delta(3),
        Self::delta(4),
        Self::delta(5),
        Self::delta(6),
        Self::delta(7),
        Self::delta(8),
        Self::delta(9),
        Self::delta(10),
        Self::delta(11),
        Self::delta(12),
        Self::delta(13),
        Self::delta(14),
        Self::delta(15),
        Self::delta(16),
        Self::delta(17),
        Self::delta(18),
        Self::delta(19),
        Self::delta(20),
        Self::delta(21),
        Self::delta(22),
        Self::delta(23),
        Self::delta(24),
        Self::delta(25),
        Self::delta(26),
    ];

    // cos_quad = math.cos(2*math.asin(1/ELE_PER_SIDE))
    pub fn of_vector(v: &Vec3) -> Self {
        let xe =
            ((v[0] + 1.0).abs() * (Self::ELE_PER_SIDE as f32) / 2.0 * 0.99999).floor() as usize;
        let ye =
            ((v[1] + 1.0).abs() * (Self::ELE_PER_SIDE as f32) / 2.0 * 0.99999).floor() as usize;
        let ze =
            ((v[1] + 1.0).abs() * (Self::ELE_PER_SIDE as f32) / 2.0 * 0.99999).floor() as usize;
        Self(xe + ye * Self::ELE_PER_SIDE + ze * Self::ELE_PER_SIDE2)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn neighbors(&self) -> SubcubeMask {
        let mut mask = SubcubeMask::ALL;
        let q = self.as_usize();
        let x = q % Self::ELE_PER_SIDE;
        let y = (q / Self::ELE_PER_SIDE) % Self::ELE_PER_SIDE;
        let z = q / Self::ELE_PER_SIDE2;

        if x == 0 {
            mask = mask | SubcubeMask::MASK_XL;
        }
        if x == Self::ELE_PER_SIDE - 1 {
            mask = mask | SubcubeMask::MASK_XR;
        }

        if y == 0 {
            mask = mask | SubcubeMask::MASK_YL;
        }
        if y == Self::ELE_PER_SIDE - 1 {
            mask = mask | SubcubeMask::MASK_YR;
        }

        if z == 0 {
            mask = mask | SubcubeMask::MASK_ZL;
        }
        if z == Self::ELE_PER_SIDE - 1 {
            mask = mask | SubcubeMask::MASK_ZR;
        }
        mask
    }

    pub fn iter_neighbors(&self) -> SubcubeNeighborIter {
        let mask = self.neighbors();
        SubcubeNeighborIter {
            s: *self,
            delta_index: 0,
            mask,
        }
    }
}

impl std::ops::Add<isize> for Subcube {
    type Output = Subcube;
    fn add(self, delta: isize) -> Subcube {
        let s = self.0 as isize + delta;
        assert!(
            s >= 0,
            "Delta of Subcube used to take subcube out of bounds"
        );
        Subcube(s as usize)
    }
}

pub struct SubcubeNeighborIter {
    s: Subcube,
    delta_index: usize,
    mask: SubcubeMask,
}
impl std::iter::Iterator for SubcubeNeighborIter {
    type Item = Subcube;
    fn next(&mut self) -> Option<Subcube> {
        while self.delta_index < 27 {
            let di = self.delta_index;
            self.delta_index += 1;
            if !self.mask.disable(di) {
                return Some(self.s + Subcube::DELTAS[di]);
            }
        }
        None
    }
}
