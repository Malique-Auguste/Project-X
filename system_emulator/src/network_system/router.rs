use std::collections::hash_map::HashMap;

use super::devices::device::Device;
use super::network_node::*;
use crate::helper::rand_u8_with_exclusion;

#[derive(Clone, PartialEq, Eq)]
pub struct Router {
    ip_address: IpAddress,
    name: String,
    password: String,
    status: Status,

    devices: HashMap<IpAddress, Device>,
    /*
    Caches the device value of new ip_adresses to avoid having to generate new values everytime,
    because this process is resource intensive
    */
    new_devices_cache: Vec<u8>,
}

impl Router {
    pub fn new(
        ip_address: IpAddress,
        name: String,
        password: String,
        status: Status,
        devices: HashMap<IpAddress, Device>,
    ) -> Router {
        Router {
            ip_address,
            name,
            password,
            status,
            devices,
            new_devices_cache: Vec::with_capacity(8),
        }
    }

    //generates a random ip for a new device that hasn't been used before
    pub fn gen_new_ip_address(&mut self) -> Result<IpAddress, &'static str> {
        if self.devices.len() >= 255 {
            Err("A maximum number of devices are already connected to the router")
        } else {
            let device = match self.new_devices_cache.pop() {
                Some(x) => x,
                None => {
                    let taken_devices: Vec<u8> = self.devices.keys().map(|k| k.device()).collect();
                    self.new_devices_cache = rand_u8_with_exclusion(taken_devices, 8);
                    self.new_devices_cache.pop().unwrap()
                }
            };

            Ok(IpAddress::new(
                u16::from_be_bytes(self.ip_address.location()),
                self.ip_address.router(),
                device,
            ))
        }
    }

    pub fn get_device(&mut self, ip_address: &IpAddress) -> Option<&Device> {
        self.devices.get(ip_address)
    }

    pub fn get_devices(&mut self) -> Vec<&Device> {
        self.devices.values().collect()
    }

    pub fn get_device_mut(&mut self, ip_address: &IpAddress) -> Option<&mut Device> {
        self.devices.get_mut(ip_address)
    }

    pub fn add_device(&mut self, mut device: Device) -> Result<(), &'static str> {
        let ip_address = match self.gen_new_ip_address() {
            Ok(ip) => ip,
            Err(e) => return Err(e),
        };

        device.set_ip_address(ip_address);
        self.devices.insert(*device.get_ip_address(), device);
        Ok(())
    }
}

impl NetworkNode for Router {
    fn get_ip_address(&self) -> &IpAddress {
        &self.ip_address
    }

    fn set_ip_address(&mut self, ip_address: IpAddress) {
        self.ip_address = ip_address;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_password(&self) -> &str {
        &self.password
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    fn get_status(&self) -> &Status {
        &self.status
    }

    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}
