use rand::RngCore;

use crate::data::job::{DESCRIPTOR, LEVEL, TITLE};
use crate::{Unreal, choose};

/// Generate random job data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random job title from the job title data set.
        pub fn title(&mut self) from TITLE;
        ///  Return a random job descriptor from the job descriptor data set.
        pub fn descriptor(&mut self) from DESCRIPTOR;
        /// Return a random job level from the job level data set.
        pub fn level(&mut self) from LEVEL;
    }
}
