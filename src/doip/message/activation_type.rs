use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationType {
    Default = 0x00,
    WwhObd = 0x01,
    CentralSecurity = 0x02,
}

impl fmt::Display for ActivationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let activation_string = match self {
            ActivationType::Default => "Default",
            ActivationType::WwhObd => "WWH-OBD",
            ActivationType::CentralSecurity => "Central security",
        };
        write!(f, "{}", activation_string)
    }
}
