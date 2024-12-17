use rand::RngCore;

use crate::data::files::{EXTENSION, MIME_TYPE};
use crate::{Unreal, choose};

/// Generate random file data.
impl<R: RngCore> Unreal<R> {
    choose! {
        /// Return a random MIME type from the MIME type data set.
        pub fn mime_type(&mut self) from MIME_TYPE;
        /// Return a random file extension from the file extension data set.
        pub fn extension(&mut self) from EXTENSION;
    }
}
