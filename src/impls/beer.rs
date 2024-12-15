use rand::{Rng, RngCore};

use crate::data::beer::{HOP, MALT, NAME, STYLE, YEAST};
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn name(&mut self) from NAME;
        pub fn style(&mut self) from STYLE;
        pub fn hop(&mut self) from HOP;
        pub fn yeast(&mut self) from YEAST;
        pub fn malt(&mut self) from MALT;
    }

    pub fn bitterness(&mut self) -> String {
        format!("{} IBU", self.gen_range(10..=100))
    }

    pub fn percentage(&mut self) -> String {
        format!("{}%", self.gen_range(2. ..=10.))
    }

    pub fn balling(&mut self) -> String {
        format!("{}°Blg", self.gen_range(5. ..=20.))
    }
}
