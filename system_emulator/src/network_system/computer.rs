use crate::file_system::directory::Directory;
use crate::network_system::ip_address::IpAddress;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Computer {
    ip_address: IpAddress,
    pub storage: Directory,
}

impl Computer {
    pub fn new(storage: Directory, ip_address: IpAddress) -> Computer {
        Computer{storage, ip_address}
    }
}