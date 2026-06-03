use super::{DrawingInstruction, DrawingInstructionIterator, PtIndex};

/// A drawing, for a specific level of detail of a [Constellation], that consists of a list of [DrawingInstruction]s
pub struct Drawing<'a> {
    pub instructions: &'a [DrawingInstruction],
}

impl<'a> Drawing<'a> {
    /// Iterate therough the instructions, interpreting them, to produce commands to draw lines and curves
    pub(crate) fn iter<F>(&self, f: F) -> impl Iterator<Item = (usize, [[f64; 3]; 4])>
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
