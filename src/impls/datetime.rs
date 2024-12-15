use chrono::DateTime;
use chrono::{Datelike, Local, NaiveDateTime};
use rand::{Rng, RngCore};

use crate::Unreal;
use crate::choose;
use crate::data::datetime::{ABR, FULL, OFFSET, TEXT};
use std::ops::{Bound, RangeBounds};

impl<R: RngCore> Unreal<R> {
    pub fn month(&mut self) -> u32 {
        self.gen_range(1..=12)
    }

    pub fn day(&mut self) -> u32 {
        self.gen_range(1..=28)
    }

    pub fn week_day(&mut self) -> u32 {
        self.gen_range(0..=6)
    }

    pub fn year(&mut self) -> i32 {
        self.gen_range(1980..=Local::now().year())
    }

    pub fn hour(&mut self) -> u32 {
        self.gen_range(0..=23)
    }

    pub fn minute(&mut self) -> u32 {
        self.gen_range(0..=59)
    }

    pub fn second(&mut self) -> u32 {
        self.gen_range(0..=59)
    }

    pub fn nanosecond(&mut self) -> u32 {
        self.gen_range(0..=999_999_999)
    }

    choose! {
        pub fn timezone(&mut self) from TEXT;
        pub fn timezone_full(&mut self) from FULL;
        pub fn timezone_abv(&mut self) from ABR;
        pub fn timezone_offset(&mut self) from OFFSET;
    }

    pub fn datetime_in_range(
        &mut self,
        range: impl RangeBounds<NaiveDateTime>,
    ) -> Option<NaiveDateTime> {
        let start_nanos = match range.start_bound() {
            Bound::Included(start) => start.and_utc().timestamp_nanos_opt()?,
            Bound::Excluded(start) => start.and_utc().timestamp_nanos_opt()? + 1,
            Bound::Unbounded => return None,
        };
        let end_nanos = match range.end_bound() {
            Bound::Included(end) => end.and_utc().timestamp_nanos_opt()? + 1,
            Bound::Excluded(end) => end.and_utc().timestamp_nanos_opt()?,
            Bound::Unbounded => return None,
        };

        Some(DateTime::from_timestamp_nanos(self.gen_range(start_nanos..end_nanos)).naive_local())
    }

    pub fn datetime(&mut self) -> Option<NaiveDateTime> {
        Some(
            DateTime::from_timestamp_nanos(self.gen_range(0..Local::now().timestamp_nanos_opt()?))
                .naive_local(),
        )
    }
}
