use super::{DrawingInstruction, DrawingInstructionIterator, DrawingOperation, PtIndex};

/// A drawing, for a specific level of detail of a [Constellation], that consists of a list of [DrawingInstruction]s
pub struct Drawing<'a> {
    pub instructions: &'a [DrawingInstruction],
}

impl<'a> Drawing<'a> {
    /// Iterate therough the instructions, interpreting them, to produce commands to draw lines and curves
    pub(crate) fn iter<'iter, F>(&self, f: F) -> impl Iterator<Item = DrawingOperation> + 'iter
    where
        F: Fn(PtIndex) -> Option<[f64; 3]> + 'iter,
        'a: 'iter,
    {
        DrawingInstructionIterator::new(f, self.instructions.iter().copied())
    }
}
