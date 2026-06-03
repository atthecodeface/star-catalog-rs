//!
//!
//! Constants that represent some common constellations, using Hipparcos numbers for the stars

mod constellation;
mod drawing;
mod drawing_instructions;

pub use constellation::Constellation;
pub use drawing::Drawing;
pub(crate) use drawing_instructions::DrawingInstructionIterator;
pub use drawing_instructions::{DrawingInstruction, PtIndex};
