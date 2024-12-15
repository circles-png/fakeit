use rand::{Rng, RngCore};
use std::fmt::Write;

use crate::data::color::{FULL, SAFE};
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn full_colour_name(&mut self) from FULL;
        pub fn safe_colour_name(&mut self) from SAFE;
    }

    #[must_use]
    pub fn rgb24_hex(&mut self) -> String {
        self.rgb24()
            .into_iter()
            .fold(String::new(), |mut string, byte| {
                write!(string, "{byte:02x}").expect("should be able to write to string");
                string
            })
    }

    #[must_use]
    pub fn rgb24(&mut self) -> [u8; 3] {
        self.r#gen()
    }
}
