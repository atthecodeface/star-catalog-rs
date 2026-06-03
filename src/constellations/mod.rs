//!
//!
//! Constants that represent some common constellations, using Hipparcos numbers for the stars

/// Index into the points of a constellation
#[derive(Clone, Copy)]
pub struct PtIndex(pub u8);

/// An instruction for drawing, provided by constellations
///
/// These are interpreted internally and can be iterated through to provide a
/// client with instructions to draw lines or curves
#[derive(Clone, Copy)]
pub enum DrawingInstruction {
    /// Set last point to the specified ID; clears in-hand data
    MoveTo(PtIndex),
    /// Draw from last point to the specified ID and set last point; clears in-hand data
    DrawTo(PtIndex),
    /// Draw a curve with specified number of control points (max 4); sets last point to the end control point; clears data
    ///
    /// Draw of <2 just clears and sets the last point
    Draw(u8),
    /// Add, to the in-hand pt specified, the constellation star, with a given weight
    AddWeightedPoint(u8, PtIndex, f32),
}

/// An iterator that converts [DrawingInstruction] into curve drawing commands
struct DrawingInstructionIterator<F, I>
where
    F: Fn(PtIndex) -> Option<[f64; 3]>,
    I: Iterator<Item = DrawingInstruction>,
{
    f: F,
    instructions: I,
    pending_clear: bool,
    pts: [[f64; 3]; 4],
}

impl<F, I> Iterator for DrawingInstructionIterator<F, I>
where
    F: Fn(PtIndex) -> Option<[f64; 3]>,
    I: Iterator<Item = DrawingInstruction>,
{
    type Item = (usize, [[f64; 3]; 4]);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pending_clear {
            self.pts[0] = self.pts[3];
            self.pts[1] = [0.0; 3];
            self.pts[2] = [0.0; 3];
            self.pts[3] = [0.0; 3];
            self.pending_clear = false;
        }
        while let Some(inst) = self.instructions.next() {
            match inst {
                DrawingInstruction::MoveTo(pt) => {
                    let Some(pt) = (self.f)(pt) else {
                        eprintln!("Argh failed to find pt {}", pt.0);
                        return None;
                    };
                    self.pts[0] = pt;
                }
                DrawingInstruction::DrawTo(pt) => {
                    let Some(pt) = (self.f)(pt) else {
                        eprintln!("Argh failed to find pt {}", pt.0);
                        return None;
                    };
                    self.pts[1] = pt;
                    self.pts[3] = pt;
                    self.pending_clear = true;
                    return Some((2, self.pts));
                }
                DrawingInstruction::Draw(n) => {
                    let n = n as usize;
                    self.pending_clear = true;
                    if n < 4 {
                        self.pts[3] = self.pts[n - 1];
                    }
                    return Some((n, self.pts));
                }
                DrawingInstruction::AddWeightedPoint(n, pt, weight) => {
                    let Some(pt) = (self.f)(pt) else {
                        return None;
                    };
                    for (i, p) in self.pts[n as usize].iter_mut().zip(pt.iter()) {
                        *i += (weight as f64) * *p;
                    }
                }
            }
        }
        None
    }
}

/// A drawing, for a specific level of detail of a [Constellation], that consists of a list of [DrawingInstruction]s
pub struct Drawing<'a> {
    pub instructions: &'a [DrawingInstruction],
}

impl<'a> Drawing<'a> {
    /// Iterate therough the instructions, interpreting them, to produce commands to draw lines and curves
    fn iter<F>(&self, f: F) -> impl Iterator<Item = (usize, [[f64; 3]; 4])>
    where
        F: Fn(PtIndex) -> Option<[f64; 3]>,
    {
        DrawingInstructionIterator {
            f: f,
            instructions: self.instructions.iter().copied(),
            pending_clear: true,
            pts: [[0.0; 3]; 4],
        }
    }
}

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
    pub fn instructions(
        &self,
        catalog: &crate::Catalog,
        lod: usize,
    ) -> Option<impl Iterator<Item = (usize, [[f64; 3]; 4])>> {
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
