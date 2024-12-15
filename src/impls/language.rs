use rand::RngCore;

use crate::data::language::{LONG, PROGRAMMING, SHORT};
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn language_long(&mut self) from LONG;
        pub fn language_short(&mut self) from SHORT;
        pub fn programming(&mut self) from PROGRAMMING;
    }
}
