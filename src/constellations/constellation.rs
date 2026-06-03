use super::{Drawing, DrawingOperation, PtIndex};

/// A constellation description consisting of a name, the points required (ids
/// within the catalog it belongs to), and drawings at various levels of detail
///
/// Level of detail 0 *must* be supplied, and it must connect the stars in the
/// constellation; higher levels of detail can be more 'artistic'
pub struct Constellation<'a> {
    /// The name of the constellation
    pub name: &'a str,
    /// The catalog ids of the points in the constellation
    pub points: &'a [usize],
    /// The drawings for different levels of detail for the constellation
    pub drawings: &'a [Drawing<'a>],
}

impl<'a> std::ops::Index<PtIndex> for Constellation<'a> {
    type Output = usize;
    #[track_caller]
    fn index(&self, index: PtIndex) -> &Self::Output {
        &self.points[index.0 as usize]
    }
}

impl<'a> Constellation<'a> {
    /// Returns the name of the constellation
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the list of catalog ids of the points used in the constellation
    pub fn points(&self) -> &[usize] {
        &self.points
    }

    /// Returns the number of levels of detail; this will be at least one
    pub fn levels_of_detail(&self) -> usize {
        self.drawings.len()
    }

    /// Returns the drawing at a specific level of detail, or none if that level
    /// of detail has not been supplied by the constellation
    pub fn drawing(&self, lod: usize) -> Option<&Drawing<'_>> {
        if lod < self.drawings.len() {
            Some(&self.drawings[lod])
        } else {
            None
        }
    }

    /// Iterates through the drawing instructions for a level of detail
    /// providing drawing operations to be performed
    pub fn instructions<'inst>(
        &'inst self,
        catalog: &'inst crate::Catalog,
        lod: usize,
    ) -> Option<impl Iterator<Item = DrawingOperation> + 'inst> {
        if lod >= self.drawings.len() {
            return None;
        }
        Some(self.drawings[lod].iter(|pt| {
            catalog
                .find_sorted(self[pt])
                .map(|index| *catalog[index].vector())
        }))
    }
}
