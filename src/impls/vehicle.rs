use rand::RngCore;

use crate::{
    Unreal, choose,
    data::vehicle::{FUEL_TYPE, MAKER, MODEL, TRANSMISSION_TYPE, TYPE},
};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn vehicle_type(&mut self) from TYPE;
        pub fn fuel(&mut self) from FUEL_TYPE;
        pub fn transmission_gear(&mut self) from TRANSMISSION_TYPE;
        pub fn car_maker(&mut self) from MAKER;
        pub fn car_model(&mut self) from MODEL;
    }
}
