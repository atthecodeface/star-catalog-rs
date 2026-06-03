// To do
//
// Convert iterator to return line, curve, set style instead of usize + pts; probably pass in points array
//
// Add ability to 'look at' with 'up'
//
// Add ability to move to position (u16,16) and draw to position (u16,u16)
//
// Add ability to set style

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
pub struct DrawingInstructionIterator<F, I>
where
    F: Fn(PtIndex) -> Option<[f64; 3]>,
    I: Iterator<Item = DrawingInstruction>,
{
    pub f: F,
    pub instructions: I,
    pub pending_clear: bool,
    pub pts: [[f64; 3]; 4],
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
