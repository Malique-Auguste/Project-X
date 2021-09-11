use std::fmt;

//All objects that can be a part of a network must implement NetworkNode
pub trait NetworkNode {
    fn get_ip_address(&self) -> &IpAddress;
    fn set_ip_address(&mut self, ip_address: IpAddress);
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: String);
    fn get_password(&self) -> &str;
    fn set_password(&mut self, password: String);
    fn get_status(&self) -> &Status;
    fn set_status(&mut self, status: Status);
}

impl fmt::Display for dyn NetworkNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.get_name(),
            self.get_ip_address(),
            self.get_status()
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Status {
    Online,
    Offline,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Online => write!(f, "Online"),
            Status::Offline => write!(f, "Offline"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct IpAddress {
    location: [u8; 2],
    router: u8,
    device: u8,
}

impl IpAddress {
    pub fn new(location: u16, router: u8, device: u8) -> IpAddress {
        IpAddress {
            location: location.to_be_bytes(),
            router,
            device,
        }
    }

    pub fn location(&self) -> [u8; 2] {
        self.location
    }

    pub fn router(&self) -> u8 {
        self.router
    }

    pub fn device(&self) -> u8 {
        self.device
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.location[0], self.location[1], self.router, self.device
        )
    }
}
