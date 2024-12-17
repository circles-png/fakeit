use rand::RngCore;
use rand::seq::SliceRandom;

use crate::Unreal;
use crate::data::status_code::{GENERAL, SIMPLE};

/// Generate random status code data.
impl<R: RngCore> Unreal<R> {
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    /// Return a random simple status code, from the simple status code data set.
    pub fn simple_status_code(&mut self) -> u32 {
        *SIMPLE.choose(self).expect("SIMPLE should not be empty")
    }

    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    /// Return a random general status code, from the general status code data set.
    pub fn general_status_code(&mut self) -> u32 {
        *GENERAL.choose(self).expect("GENERAL should not be empty")
    }
}
