use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::hipster::WORD;

/// Generate random hipster data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random hipster word from the hipster word data set.
        pub fn hipster_word(&mut self) from WORD;
    }
}
