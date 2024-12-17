use rand::{Rng, RngCore};

use crate::data::beer::{HOP, MALT, NAME, STYLE, YEAST};
use crate::{Unreal, choose};

/// Generate random beer data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random beer name from the beer name data set.
        pub fn beer_name(&mut self) from NAME;
        /// Return a random beer style from the beer style data set.
        pub fn beer_style(&mut self) from STYLE;
        /// Return a random beer hop from the hop data set.
        pub fn beer_hop(&mut self) from HOP;
        /// Return a random beer yeast from the yeast data set.
        pub fn beer_yeast(&mut self) from YEAST;
        /// Return a random beer malt from the malt data set.
        pub fn malt(&mut self) from MALT;
    }

    /// Return a random beer bitterness.
    pub fn beer_bitterness(&mut self) -> String {
        format!("{} IBU", self.gen_range(10..=100))
    }

    /// Return a random beer alcohol percentage.
    pub fn beer_alcohol(&mut self) -> String {
        format!("{}%", self.gen_range(2. ..=10.))
    }

    /// Return a random beer Balling value.
    pub fn beer_balling(&mut self) -> String {
        format!("{}°Blg", self.gen_range(5. ..=20.))
    }
}
