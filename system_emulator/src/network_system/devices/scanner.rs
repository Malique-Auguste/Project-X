use crate::network_system::network_node::{IpAddress, Status};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Scanner {
    pub(super) ip_address: IpAddress,
    pub(super) name: String,
    pub(super) password: String,
    pub(super) status: Status,
}

impl Scanner {
    pub fn new(ip_address: IpAddress, name: String, password: String, status: Status) -> Scanner {
        Scanner {
            ip_address,
            name,
            password,
            status,
        }
    }
}
