use rand::RngCore;

use crate::data::animal::{ANIMAL, CAT, DOG, FARM, PET_NAME, TYPE};
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn pet_name(&mut self) from PET_NAME;
        pub fn animal(&mut self) from ANIMAL;
        pub fn type_of(&mut self) from TYPE;
        pub fn farm(&mut self) from FARM;
        pub fn cat(&mut self) from CAT;
        pub fn dog(&mut self) from DOG;
    }
}
