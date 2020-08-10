use crate::look_and_model::recording::{CapturedMove, FireAddress};

impl CapturedMove {
    pub fn new(source: FireAddress, destination: FireAddress) -> Self {
        CapturedMove {
            source: source,
            destination: destination,
        }
    }
}
