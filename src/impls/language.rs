use rand::RngCore;

use crate::data::language::{LONG, PROGRAMMING, SHORT};
use crate::{Unreal, choose};

/// Generate random language data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random language from the language data set.
        pub fn language_long(&mut self) from LONG;
        /// Return a random language abbreviation from the language abbreviation data set.
        pub fn language_short(&mut self) from SHORT;
        /// Return a random programming language from the programming language data set.
        pub fn programming(&mut self) from PROGRAMMING;
    }
}
