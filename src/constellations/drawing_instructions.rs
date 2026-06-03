// To do
//
// Convert iterator to return line, curve, set style instead of usize + pts; probably pass in points array
//
// Add ability to 'look at' with 'up'
//
// Add ability to move to position (u16,16) and draw to position (u16,u16)
//
// Add ability to set style

use crate::{Vec3, Vector};

/// Index into the points of a constellation
#[derive(Clone, Copy)]
pub struct PtIndex(pub u8);

/// Drawing operation
#[derive(Clone, Copy)]
pub enum DrawingOperation {
    /// Straight line between the two points
    Line([[f64; 3]; 2]),
    /// Quadratic Bezier curve using the first three points (endpoints are 0 and 2)
    QuadBezier([[f64; 3]; 3]),
    /// Cubic Bezier curve using the all four points (endpoints are 0 and 3)
    CubicBezier([[f64; 3]; 4]),
    /// Set the style in some manner
    Style,
}

/// An instruction for drawing, provided by constellations
///
/// These are interpreted internally and can be iterated through to provide a
/// client with instructions to draw lines or curves
#[derive(Clone, Copy)]
pub enum DrawingInstruction {
    /// Look at a point with 'up' as the second point, with a scale (i.e. divide the distance between those two points into 'n' bits)
    LookAt(PtIndex, PtIndex, u16),
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
    /// Set point u8 to be an X and Y amount (of the axes, scaled by xy_scale)
    SetXY(u8, i16, i16),
    /// Set point u8 to be an X and Y amount (of the axes, scaled by xy_scale) and then draw a curve with that as the endpoint; clears data
    DrawXY(u8, i16, i16),
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
    pub look_at: Vec3,
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub xy_scale: f64,
    pub pts: [Vec3; 4],
}

impl<F, I> DrawingInstructionIterator<F, I>
where
    F: Fn(PtIndex) -> Option<[f64; 3]>,
    I: Iterator<Item = DrawingInstruction>,
{
    pub fn new(f: F, instructions: I) -> Self {
        Self {
            f,
            instructions,
            pending_clear: true,
            look_at: Vec3::default(),
            y_axis: Vec3::default(),
            x_axis: Vec3::default(),
            xy_scale: 1.0 / 256.0,
            pts: [Vec3::default(); 4],
        }
    }
}

impl<F, I> DrawingInstructionIterator<F, I>
where
    F: Fn(PtIndex) -> Option<[f64; 3]>,
    I: Iterator<Item = DrawingInstruction>,
{
    fn vec_of_pt(&self, pt: PtIndex) -> Option<Vec3> {
        let Some(pt) = (self.f)(pt) else {
            eprintln!("Argh failed to find pt {}", pt.0);
            return None;
        };
        Some(pt.into())
    }
}

impl<F, I> Iterator for DrawingInstructionIterator<F, I>
where
    F: Fn(PtIndex) -> Option<[f64; 3]>,
    I: Iterator<Item = DrawingInstruction>,
{
    type Item = DrawingOperation;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pending_clear {
            self.pts[0] = self.pts[3];
            self.pts[1] = Vec3::default();
            self.pts[2] = Vec3::default();
            self.pts[3] = Vec3::default();
            self.pending_clear = false;
        }
        while let Some(inst) = self.instructions.next() {
            match inst {
                DrawingInstruction::LookAt(at_pt, up_pt, scale) => {
                    let at_pt = self.vec_of_pt(at_pt)?;
                    let up_dir = self.vec_of_pt(up_pt)? - at_pt;
                    self.look_at = at_pt.normalize();
                    let up_len = up_dir.length();
                    self.x_axis = up_dir.normalize();
                    self.y_axis = self.look_at.cross_product(&self.x_axis).normalize();
                    self.x_axis *= up_len / (scale as f64);
                    self.y_axis *= up_len / (scale as f64);
                }
                DrawingInstruction::MoveTo(pt) => {
                    let Some(pt) = (self.f)(pt) else {
                        eprintln!("Argh failed to find pt {}", pt.0);
                        return None;
                    };
                    self.pts[0] = pt.into();
                }
                DrawingInstruction::DrawTo(pt) => {
                    let Some(pt) = (self.f)(pt) else {
                        eprintln!("Argh failed to find pt {}", pt.0);
                        return None;
                    };
                    self.pts[3] = pt.into();
                    self.pending_clear = true;
                    return Some(DrawingOperation::Line([*self.pts[0], pt]));
                }
                DrawingInstruction::SetXY(n, x, y) => {
                    let n = n as usize;
                    self.pts[n] =
                        (self.look_at + self.x_axis * (x as f64) + self.y_axis * (y as f64))
                            .normalize();
                }
                DrawingInstruction::DrawXY(n, x, y) => {
                    let n = n as usize;
                    self.pts[n - 1] =
                        (self.look_at + self.x_axis * (x as f64) + self.y_axis * (y as f64))
                            .normalize();

                    self.pending_clear = true;
                    if n < 4 {
                        self.pts[3] = self.pts[n - 1];
                    }
                    return match n {
                        3 => Some(DrawingOperation::QuadBezier([
                            *self.pts[0],
                            *self.pts[1],
                            *self.pts[3],
                        ])),
                        4 => Some(DrawingOperation::CubicBezier([
                            *self.pts[0],
                            *self.pts[1],
                            *self.pts[2],
                            *self.pts[3],
                        ])),
                        _ => Some(DrawingOperation::Line([*self.pts[0], *self.pts[3]])),
                    };
                }
                DrawingInstruction::Draw(n) => {
                    let n = n as usize;
                    self.pending_clear = true;
                    if n < 4 {
                        self.pts[3] = self.pts[n - 1];
                    }
                    return match n {
                        3 => Some(DrawingOperation::QuadBezier([
                            *self.pts[0],
                            *self.pts[1],
                            *self.pts[3],
                        ])),
                        4 => Some(DrawingOperation::CubicBezier([
                            *self.pts[0],
                            *self.pts[1],
                            *self.pts[2],
                            *self.pts[3],
                        ])),
                        _ => Some(DrawingOperation::Line([*self.pts[0], *self.pts[3]])),
                    };
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
