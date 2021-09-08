use std::collections::{HashMap};

use crate::network_system::ip_address::{IpAddress, HasIP};
use crate::network_system::device::Device;
use crate::helper::rand_u8_with_exclusion;

#[derive(Clone, PartialEq, Eq)]
pub struct Switch {
    devices: HashMap<IpAddress, Device>,
    ip_address: IpAddress,
    new_devices_cache: Vec<u8>
}

impl Switch {
    pub fn new(ip_address: IpAddress, devices: HashMap<IpAddress, Device>, ) -> Switch {
        Switch {devices, ip_address, new_devices_cache: Vec::with_capacity(8)}
    }

    pub fn gen_new_ip_address(&mut self) -> Result<IpAddress, ()> {
        if self.devices.len() >= 255 {
            //a maximum number of devices are already connected to the switch
            return Err(())
        }
        else {
            let device = match self.new_devices_cache.pop() {
                Some(x) => x,
                None => {
                    let taken_devices: Vec<u8> = self.devices.keys().map(|k| k.device()).collect();
                    self.new_devices_cache = rand_u8_with_exclusion(taken_devices, 8);
                    self.new_devices_cache.pop().unwrap()
                }
            };

            return Ok(IpAddress::new(u16::from_be_bytes(self.ip_address.router()), self.ip_address.switch(), device))
        }
    }

    pub fn get_device(&mut self, ip_address: &IpAddress) -> Option<&Device> {
        self.devices.get(ip_address)
    }

    pub fn get_device_mut(&mut self, ip_address: &IpAddress) -> Option<&mut Device> {
        self.devices.get_mut(ip_address)
    }

    pub fn add_device(&mut self, mut device: Device) -> Result<(), &str> {
        let ip_address = self.gen_new_ip_address();
        let ip_address = match ip_address {
            Ok(ip) => ip,
            Err(_) => return Err("A maximum number of devices are already connected to the switch")
        };

        device.set_ip_address(ip_address);
        self.devices.insert(*device.get_ip_address(), device);
        Ok(())
    }
}

impl HasIP for Switch {
    fn get_ip_address(&self) -> &IpAddress {
        &self.ip_address
    }

    fn set_ip_address(&mut self, ip_address: IpAddress) {
        self.ip_address = ip_address;
    }
}