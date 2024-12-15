use std::ops::RangeInclusive;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use unreal::Unreal;

#[test]
fn main() {
    const DATE_RANGE: RangeInclusive<NaiveDateTime> = NaiveDateTime::UNIX_EPOCH
        ..=NaiveDate::from_yo_opt(2024, 1)
            .unwrap()
            .and_time(NaiveTime::MIN);
    let mut unreal = Unreal::from_stdrng_seed(0);
    assert_eq!(unreal.credit_card_luhn_number(), "6565821702776751");
    assert_eq!(unreal.full_name(), "Vivienne Stark");
    assert_eq!(
        unreal.datetime_in_range(DATE_RANGE).unwrap().to_string(),
        "1985-04-03 04:44:58.239231046"
    );
    assert_eq!(unreal.language_long(), "Chamorro");
    assert_eq!(unreal.bs(), "exploit");

    assert_eq!(unreal.credit_card_luhn_number(), "4464199235617842");
    assert_eq!(unreal.full_name(), "Conor Blanda");
    assert_eq!(
        unreal.datetime_in_range(DATE_RANGE).unwrap().to_string(),
        "2017-07-16 01:11:19.311212830"
    );
    assert_eq!(unreal.language_long(), "Sardinian");
    assert_eq!(unreal.bs(), "visualize");
}
