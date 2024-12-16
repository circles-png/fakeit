use crate::array_consts;

array_consts![
    pub const GENERAL: [&str; _] = ["error", "warning", "info", "fatal", "trace", "debug"];

    pub const SYSLOG: [&str; _] = [
        "emerg", "alert", "crit", "err", "warning", "notice", "info", "debug",
    ];

    pub const APACHE: [&str; _] = [
        "emerg", "alert", "crit", "error", "warn", "notice", "info", "debug", "trace1-8",
    ];
];
