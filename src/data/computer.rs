use crate::array_consts;

array_consts![
    pub const LINUX_PROCESSOR: [&str; _] = ["i686", "x86_64"];

    pub const MAC_PROCESSOR: [&str; _] = ["Intel", "PPC", "U; Intel", "U; PPC"];

    pub const WINDOWS_PLATFORM: [&str; _] = [
        "Windows NT 6.2",
        "Windows NT 6.1",
        "Windows NT 6.0",
        "Windows NT 5.2",
        "Windows NT 5.1",
        "Windows NT 5.01",
        "Windows NT 5.0",
        "Windows NT 4.0",
        "Windows 98; Win 9x 4.90",
        "Windows 98",
        "Windows 95",
        "Windows CE",
    ];
];
