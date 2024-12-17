use rand::RngCore;

use crate::data::person::{FIRST, LAST, PREFIX, SUFFIX};
use crate::{Unreal, choose};

/// Generate random name data.
impl<R: RngCore> Unreal<R> {
    /// Return a random full name by concatenating a random first name from [`Self::first_name`]
    /// and a random last name from [`Self::last_name`], separated by a space.
    pub fn full_name(&mut self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }

    choose! {
        /// Return a random first name from the first name data set.
        pub fn first_name(&mut self) from FIRST;
        /// Return a random last name from the last name data set.
        pub fn last_name(&mut self) from LAST;
        /// Return a random name prefix from the name prefix data set.
        pub fn name_prefix(&mut self) from PREFIX;
        /// Return a random name suffix from the name suffix data set.
        pub fn name_suffix(&mut self) from SUFFIX;
    }
}
