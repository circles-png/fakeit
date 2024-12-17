use rand::RngCore;

use crate::data::animal::{ANIMAL, CAT, DOG, FARM, PET_NAME, TYPE};
use crate::{Unreal, choose};

/// Generate random animal data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random pet name from the pet name data set.
        pub fn pet_name(&mut self) from PET_NAME;
        /// Return a random animal from the animal data set.
        pub fn animal(&mut self) from ANIMAL;
        /// Return a random animal type from the animal type data set.
        pub fn animal_type(&mut self) from TYPE;
        /// Return a random farm animal from the farm animal data set.
        pub fn farm(&mut self) from FARM;
        /// Return a random cat from the cat data set.
        pub fn cat(&mut self) from CAT;
        /// Return a random dog from the dog data set.
        pub fn dog(&mut self) from DOG;
    }
}
