use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    NotReady = 0x00,
    Ready = 0x01,
    NotSupported = 0x02,
}

impl fmt::Display for PowerMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let power_strings = match self {
            PowerMode::NotReady => "Not ready",
            PowerMode::Ready => "Ready",
            PowerMode::NotSupported => "Not supported",
        };
        write!(f, "{}", power_strings)
    }
}
