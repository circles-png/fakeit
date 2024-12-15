use rand::RngCore;

use crate::Unreal;
use crate::choose;
use crate::data::hipster::WORD;

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn hipster_word(&mut self) from WORD;
    }
}
