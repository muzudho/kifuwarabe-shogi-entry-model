use crate::look_and_model::{recording::HandAddress, AbsoluteAddress2D, DoubleFacedPiece};

impl HandAddress {
    pub fn new(piece: DoubleFacedPiece, sq: AbsoluteAddress2D) -> Self {
        HandAddress {
            piece: piece,
            sq: sq,
        }
    }
}
