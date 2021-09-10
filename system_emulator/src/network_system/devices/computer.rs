use crate::network_system::network_node::{IpAddress, Status};
use crate::file_system::directory::Directory;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Computer {
    pub(super) ip_address: IpAddress,
    pub(super) name: String,
    pub(super) password: String,
    pub(super) status: Status,
    storage: Directory
}

impl Computer {
    pub fn new(ip_address: IpAddress, name: String, password: String, status: Status, storage: Directory) -> Computer {
        Computer{ip_address, name, password, status, storage}
    }
}