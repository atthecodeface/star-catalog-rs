//! This provides a [Constellation] class that describes constellations within a
//! catalog, using the 'id' values of the stars in the catalog, and it provides
//! for drawing of the constellations at various levels of detail
//!
//! Constants that represent some common constellations, using Hipparcos numbers for the stars

mod constellation;
mod drawing;
mod drawing_instructions;

pub use constellation::Constellation;
pub use drawing::Drawing;
pub(crate) use drawing_instructions::DrawingInstructionIterator;
pub use drawing_instructions::{DrawingInstruction, DrawingOperation, PtIndex};
