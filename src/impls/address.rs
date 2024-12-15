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

impl<R: RngCore> Unreal<R> {
    pub fn address(&mut self) -> String {
        format!(
            "{}, {}, {} {}",
            self.street(),
            self.city(),
            self.us_state(),
            self.zip()
        )
    }

    pub fn street(&mut self) -> String {
        once(self.street_number().as_str())
            .chain(Some(self.street_prefix()).filter(|_| self.r#gen()))
            .chain(once(self.street_name()))
            .chain(once(self.street_suffix()))
            .join(" ")
    }

    pub fn street_number(&mut self) -> String {
        self.numbers(0..=99_999, 3)
    }

    choose! {
        pub fn street_prefix(&mut self) from STREET_PREFIX;
        pub fn street_name(&mut self) from STREET_NAME;
        pub fn street_suffix(&mut self) from STREET_SUFFIX;
        pub fn us_state(&mut self) from US_STATE;
        pub fn us_state_initials(&mut self) from US_STATE_INITIALS;
        pub fn country(&mut self) from COUNTRY;
        pub fn country_abr(&mut self) from COUNTRY_ABR;
    }

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

    pub fn zip(&mut self) -> String {
        self.numbers(0..=99999, 5)
    }

    pub fn latitude(&mut self) -> f32 {
        self.gen_range(-90. ..=90.)
    }

    pub fn latitude_in_range(&mut self, range: impl SampleRange<f32>) -> f32 {
        self.gen_range(range).clamp(-90., 90.)
    }

    pub fn longitude(&mut self) -> f32 {
        self.gen_range(-180. ..=180.)
    }

    pub fn longitude_in_range(&mut self, range: impl SampleRange<f32>) -> f32 {
        self.gen_range(range).clamp(-180., 180.)
    }
}
