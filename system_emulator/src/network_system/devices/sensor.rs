use crate::network_system::network_node::{IpAddress, Status};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Sensor {
    pub(super) ip_address: IpAddress,
    pub(super) name: String,
    pub(super) password: String,
    pub(super) status: Status

}

impl Sensor {
    pub fn new(ip_address: IpAddress, name: String, password: String, status: Status) -> Sensor {
        Sensor{ip_address, name, password, status}
    }
}