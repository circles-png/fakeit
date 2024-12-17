use std::iter::once;

use crate::Unreal;
use crate::choose;
use crate::data::address::{
    COUNTRY, COUNTRY_ABR, STREET_NAME, STREET_PREFIX, STREET_SUFFIX, US_STATE, US_STATE_INITIALS,
};
use itertools::Itertools;
use rand::Rng;
use rand::RngCore;
use rand::distributions::uniform::SampleRange;

/// Generate random address data.
impl<R: RngCore> Unreal<R> {
    /// Return a random address with a [`Self::street`], [`Self::city`], [`Self::us_state`], and
    /// [`Self::zip`].
    pub fn address(&mut self) -> String {
        format!(
            "{}, {}, {} {}",
            self.street(),
            self.city(),
            self.us_state(),
            self.zip()
        )
    }

    /// Return a random street name with a [`Self::street_number`], [`Self::street_prefix`],
    /// [`Self::street_name`], and [`Self::street_suffix`].
    pub fn street(&mut self) -> String {
        once(self.street_number().as_str())
            .chain(Some(self.street_prefix()).filter(|_| self.r#gen()))
            .chain(once(self.street_name()))
            .chain(once(self.street_suffix()))
            .join(" ")
    }

    /// Return a random street number. The number is a random integer between 0 and 99,999, and
    /// left-padded with zeroes to be never less than 3 digits long.
    pub fn street_number(&mut self) -> String {
        self.numbers(0..=99_999, 3)
    }

    choose! {
        /// Return a random street prefix from the street prefix data set.
        pub fn street_prefix(&mut self) from STREET_PREFIX;
        /// Return a random street name from the street name data set.
        pub fn street_name(&mut self) from STREET_NAME;
        /// Return a random street suffix from the street suffix data set.
        pub fn street_suffix(&mut self) from STREET_SUFFIX;
        /// Return a random US state from the US state data set.
        pub fn us_state(&mut self) from US_STATE;
        /// Return a random US state abbreviation from the US state abbreviation data set.
        pub fn us_state_initials(&mut self) from US_STATE_INITIALS;
        /// Return a random country from the country data set.
        pub fn country(&mut self) from COUNTRY;
        /// Return a random country abbreviation from the country abbreviation data set.
        pub fn country_abr(&mut self) from COUNTRY_ABR;
    }

    /// Return a random city name.
    pub fn city(&mut self) -> String {
        self.choose([
            (|this: &mut Self| format!("{}{}", this.first_name(), this.street_suffix()))
                as fn(&mut Self) -> String,
            (|this: &mut Self| format!("{}{}", this.last_name(), this.street_suffix()))
                as fn(&mut Self) -> String,
            (|this: &mut Self| format!("{} {}", this.street_prefix(), this.last_name()))
                as fn(&mut Self) -> String,
        ])(self)
    }

    /// Return a random 5 digit zip code.
    pub fn zip(&mut self) -> String {
        self.digits(5)
    }

    /// Return a random latitude in `-90. ..=90.`.
    pub fn latitude(&mut self) -> f32 {
        self.gen_range(-90. ..=90.)
    }

    /// Return a random latitude in the given range, clamped to `-90. ..=90.`.
    pub fn latitude_in_range(&mut self, range: impl SampleRange<f32>) -> f32 {
        self.gen_range(range).clamp(-90., 90.)
    }

    /// Return a random longitude in `-180. ..=180.`.
    pub fn longitude(&mut self) -> f32 {
        self.gen_range(-180. ..=180.)
    }

    /// Return a random longitude in the given range, clamped to `-180. ..=180.`.
    pub fn longitude_in_range(&mut self, range: impl SampleRange<f32>) -> f32 {
        self.gen_range(range).clamp(-180., 180.)
    }
}
