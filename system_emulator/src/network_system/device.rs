use crate::network_system::ip_address::{HasIP, IpAddress};
use crate::network_system::router::Router;
use crate::network_system::switch::Switch;
use crate::network_system::computer::Computer;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Device {
    Computer(Computer)
}

impl HasIP for Device {
    fn get_ip_address(&self) -> &IpAddress {
        todo!()
    }

    fn set_ip_address(&mut self, ip_address: IpAddress) {
        todo!()
    }
}