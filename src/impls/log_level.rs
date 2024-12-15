use rand::RngCore;

use crate::data::log_level::{APACHE, GENERAL, SYSLOG};
use crate::{Unreal, choose};

impl<R: RngCore> Unreal<R> {
    choose! {
        pub fn general_log_level(&mut self) from GENERAL;
        pub fn syslog_log_level(&mut self) from SYSLOG;
        pub fn apache_log_level(&mut self) from APACHE;
    }
}
