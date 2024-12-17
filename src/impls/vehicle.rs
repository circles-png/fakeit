use rand::RngCore;

use crate::{
    Unreal, choose,
    data::vehicle::{FUEL_TYPE, MAKER, MODEL, TRANSMISSION_TYPE, TYPE},
};

/// Generate random vehicle data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random vehicle type from the vehicle type data set.
        pub fn vehicle_type(&mut self) from TYPE;
        /// Return a random fuel type from the fuel type data set.
        pub fn fuel(&mut self) from FUEL_TYPE;
        /// Return a random transmission type from the transmission data set, either `Manual` or
        /// `Automatic`.
        pub fn transmission_gear(&mut self) from TRANSMISSION_TYPE;
        /// Return a random car maker from the car maker data set.
        pub fn car_maker(&mut self) from MAKER;
        /// Return a random car model from the car model data set.
        pub fn car_model(&mut self) from MODEL;
    }
}
