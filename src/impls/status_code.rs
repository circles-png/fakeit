use rand::RngCore;
use rand::seq::SliceRandom;

use crate::Unreal;
use crate::data::status_code::{GENERAL, SIMPLE};

impl<R: RngCore> Unreal<R> {
    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    pub fn simple_status_code(&mut self) -> u32 {
        *SIMPLE.choose(self).expect("SIMPLE should not be empty")
    }

    #[allow(
        clippy::missing_panics_doc,
        reason = "this should not panic under normal circumstances"
    )]
    pub fn general_status_code(&mut self) -> u32 {
        *GENERAL.choose(self).expect("GENERAL should not be empty")
    }
}
