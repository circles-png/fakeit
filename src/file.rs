use rand::RngCore;

use crate::data::files;
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn mime_type(&mut self) from files::MIME_TYPE;
        pub fn extension(&mut self) from files::EXTENSION;
    }
}
