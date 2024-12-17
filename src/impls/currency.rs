use rand::distributions::uniform::SampleRange;
use rand::{Rng, RngCore};

use crate::data::currency::UNITS;
use crate::{Unreal, choose};

/// Generate random currency data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random currency unit from the currency unit data set. The first item of the
        /// tuple is the abbreviation for the currency, and the second item is the full name of the
        /// currency.
        pub fn unit(&mut self) -> &(&str, &str); from UNITS;
    }

    /// Return a random currency abbreviation from the currency unit data set.
    pub fn currency_short(&mut self) -> &str {
        self.unit().0
    }

    /// Return a random currency name from the currency unit data set.
    pub fn currency_long(&mut self) -> &str {
        self.unit().1
    }

    /// Return a random price within the given range. The price is rounded to two decimal places.
    pub fn price(&mut self, range: impl SampleRange<f32>) -> f32 {
        (self.gen_range(range) * 100.).floor() / 100.
    }
}
