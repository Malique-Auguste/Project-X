use std::collections::HashMap;

use crate::network_system::switch::Switch;
use crate::network_system::device::Device;
use crate::network_system::ip_address::{HasIP, IpAddress};
use crate::helper::rand_u8_with_exclusion;

#[derive(Clone, PartialEq, Eq)]
pub struct Router {
    switches: HashMap<IpAddress, Switch>,
    ip_address: IpAddress,
    new_switches_cache: Vec<u8>
}

impl Router {
    pub fn new(ip_address: IpAddress, switches: HashMap<IpAddress, Switch>) -> Router {
        Router{ip_address, switches, new_switches_cache: Vec::with_capacity(8)}
    }

    pub fn gen_new_ip_address(&mut self) -> Result<IpAddress, ()> {
        if self.switches.len() >= 255 {
            //"A maximum number of switches are already connected to the router"
            return Err(())
        }
        else {
            let switch = match self.new_switches_cache.pop() {
                Some(x) => x,
                None => {
                    let taken_devices: Vec<u8> = self.switches.keys().map(|k| k.device()).collect();
                    self.new_switches_cache = rand_u8_with_exclusion(taken_devices, 8);
                    self.new_switches_cache.pop().unwrap()
                }
            };

            return Ok(IpAddress::new(u16::from_be_bytes(self.ip_address.router()), switch, 0))
        }
    }

    pub fn get_switch(&mut self, ip_address: &IpAddress) -> Option<&Switch> {
        self.switches.get(ip_address)
    }

    pub fn get_switch_mut(&mut self, ip_address: &IpAddress) -> Option<&mut Switch> {
        self.switches.get_mut(ip_address)
    }

    pub fn add_switch(&mut self, mut switch: Switch) -> Result<(), &str> {
        let ip_address = self.gen_new_ip_address();
        let ip_address = match ip_address {
            Ok(ip) => ip,
            Err(_) => return Err("A maximum number of switches are already connected to the router")
        };
        
        switch.set_ip_address(ip_address);
        self.switches.insert(*switch.get_ip_address(), switch);
        Ok(())
    }
}

impl HasIP for Router {
    fn get_ip_address(&self) -> &IpAddress {
        &self.ip_address
    }

    fn set_ip_address(&mut self, ip_address: IpAddress) {
        self.ip_address = ip_address;
    }
}