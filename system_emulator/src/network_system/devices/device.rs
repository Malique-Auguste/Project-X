use super::{computer::Computer, nid::NID, scanner::Scanner, sensor::Sensor, sentry::Sentry};
use crate::network_system::network_node::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Device {
    Computer(Computer),
    NID(NID),
    Scanner(Scanner),
    Sensor(Sensor),
    Sentry(Sentry),
}

impl NetworkNode for Device {
    fn get_ip_address(&self) -> &IpAddress {
        match self {
            Device::Computer(comp) => &comp.ip_address,
            Device::NID(nid) => &nid.ip_address,
            Device::Scanner(scan) => &scan.ip_address,
            Device::Sensor(sens) => &sens.ip_address,
            Device::Sentry(sent) => &sent.ip_address,
        }
    }

    fn set_ip_address(&mut self, ip_address: IpAddress) {
        match self {
            Device::Computer(comp) => comp.ip_address = ip_address,
            Device::NID(nid) => nid.ip_address = ip_address,
            Device::Scanner(scan) => scan.ip_address = ip_address,
            Device::Sensor(sens) => sens.ip_address = ip_address,
            Device::Sentry(sent) => sent.ip_address = ip_address,
        }
    }

    fn get_name(&self) -> &str {
        match self {
            Device::Computer(comp) => &comp.name,
            Device::NID(nid) => &nid.name,
            Device::Scanner(scan) => &scan.name,
            Device::Sensor(sens) => &sens.name,
            Device::Sentry(sent) => &sent.name,
        }
    }

    fn set_name(&mut self, name: String) {
        match self {
            Device::Computer(comp) => comp.name = name,
            Device::NID(nid) => nid.name = name,
            Device::Scanner(scan) => scan.name = name,
            Device::Sensor(sens) => sens.name = name,
            Device::Sentry(sent) => sent.name = name,
        }
    }

    fn get_password(&self) -> &str {
        match self {
            Device::Computer(comp) => &comp.password,
            Device::NID(nid) => &nid.password,
            Device::Scanner(scan) => &scan.password,
            Device::Sensor(sens) => &sens.password,
            Device::Sentry(sent) => &sent.password,
        }
    }

    fn set_password(&mut self, password: String) {
        match self {
            Device::Computer(comp) => comp.password = password,
            Device::NID(nid) => nid.password = password,
            Device::Scanner(scan) => scan.password = password,
            Device::Sensor(sens) => sens.password = password,
            Device::Sentry(sent) => sent.password = password,
        }
    }

    fn get_status(&self) -> &Status {
        match self {
            Device::Computer(comp) => &comp.status,
            Device::NID(nid) => &nid.status,
            Device::Scanner(scan) => &scan.status,
            Device::Sensor(sens) => &sens.status,
            Device::Sentry(sent) => &sent.status,
        }
    }

    fn set_status(&mut self, status: Status) {
        match self {
            Device::Computer(comp) => comp.status = status,
            Device::NID(nid) => nid.status = status,
            Device::Scanner(scan) => scan.status = status,
            Device::Sensor(sens) => sens.status = status,
            Device::Sentry(sent) => sent.status = status,
        }
    }
}
