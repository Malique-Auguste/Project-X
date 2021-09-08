use std::fmt;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, std::hash::Hash, Default)]
pub struct IpAddress{
    router: [u8; 2],
    switch: u8,
    device: u8
}

impl IpAddress {
    pub fn new(router: u16, switch: u8, device: u8) -> IpAddress {
        IpAddress{ router: router.to_be_bytes(), switch, device}
    }

    pub fn router(&self) -> [u8; 2] {
        self.router
    }

    pub fn switch(&self) -> u8 {
        self.switch
    }

    pub fn device(&self) -> u8 {
        self.device
    }
}

impl fmt::Debug for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.router[0], self.router[1], self.switch, self.device)
    }
}

pub trait HasIP {
    fn get_ip_address(&self) -> &IpAddress;
    fn set_ip_address(&mut self, ip_address: IpAddress);
}
