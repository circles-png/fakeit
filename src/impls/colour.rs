use rand::{Rng, RngCore};
use std::fmt::Write;

use crate::data::color::{FULL, SAFE};
use crate::{Unreal, choose};

/// Generate random colour data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random full colour name from the full colour name data set.
        pub fn full_colour_name(&mut self) from FULL;
        /// Return a random safe colour name from the safe colour name data set.
        pub fn safe_colour_name(&mut self) from SAFE;
    }

    /// Return a random RGB24 colour as a hexadecimal string. The string does not include a leading
    /// `#`, and is lowercase.
    pub fn rgb24_hex(&mut self) -> String {
        self.rgb24()
            .into_iter()
            .fold(String::new(), |mut string, byte| {
                write!(string, "{byte:02x}").expect("should be able to write to string");
                string
            })
    }

    /// Return a random RGB24 colour as an array of three [`u8`]s.
    pub fn rgb24(&mut self) -> [u8; 3] {
        self.r#gen()
    }
}
