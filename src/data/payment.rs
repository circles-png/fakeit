use crate::array_consts;

array_consts![
    pub const CARD: [(&str, &[&str]); _] = [
        ("Visa", &["4"] as &[&str]),
        ("MasterCard", &["222100", "272099"]),
        ("American Express", &["34", "37"]),
        ("Discover", &["65"]),
    ];
];
