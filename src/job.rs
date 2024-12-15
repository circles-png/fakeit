use rand::RngCore;

use crate::data::job::{DESCRIPTOR, LEVEL, TITLE};
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn title(&mut self) from TITLE;
        pub fn descriptor(&mut self) from DESCRIPTOR;
        pub fn level(&mut self) from LEVEL;
    }
}
