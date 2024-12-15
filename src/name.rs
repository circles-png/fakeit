use rand::RngCore;

use crate::data::person::{FIRST, LAST, PREFIX, SUFFIX};
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    #[must_use]
    pub fn full_name(&mut self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }

    choose! {
        pub fn first_name(&mut self) from FIRST;
        pub fn last_name(&mut self) from LAST;
        pub fn prefix(&mut self) from PREFIX;
        pub fn suffix(&mut self) from SUFFIX;
    }
}
