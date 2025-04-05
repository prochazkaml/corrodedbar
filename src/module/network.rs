use crate::config;
use crate::modules;

use dbus::blocking::Connection;
use networkmanager::devices::{Any, Device};
use networkmanager::NetworkManager;

struct Network {
	dbus: Connection,
}

fn ipv4_format(ip: u32, mask: u32) -> String {
	format!("{}.{}.{}.{}/{}",
		ip & 0xFF,
		(ip >> 8) & 0xFF,
		(ip >> 16) & 0xFF,
		ip >> 24,
		mask
	)
}

impl modules::ModuleImplementation for Network {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let mut ips: Vec<String> = Vec::new();

		let nm = NetworkManager::new(&self.dbus);

		let devices = match nm.get_devices() {
			Ok(val) => val,
			Err(_) => { return Ok(None); }
		};

		for device in devices {
			let deviplist = match device {
				Device::WiFi(wifi) => match wifi.ip4_config() {
					Ok(cfg) => match cfg.addresses() { Ok(addrlist) => addrlist, Err(_) => { continue; }},
					Err(_) => { continue; }
				},
				Device::Ethernet(eth) => match eth.ip4_config() {
					Ok(cfg) => match cfg.addresses() { Ok(addrlist) => addrlist, Err(_) => { continue; }},
					Err(_) => { continue; }
				}
				_ => { continue; } // TODO - other interfaces, IPv6
			};

			for ip in &deviplist {
				ips.push(ipv4_format(ip[0], ip[1]));
			}
		}

		if ips.len() <= 0 {
			return Ok(None);
		}
		
		let mut output = String::new();

		for i in 0..ips.len() {
			output += &ips[i];

			if i < ips.len() - 1 {
				output += " ";
			}
		}

		return Ok(Some(output));
	}
}

pub fn init(_config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	// TODO - specific connection
	
	let dbus = match dbus::blocking::Connection::new_system() {
		Ok(val) => val,
		Err(_) => { return Err("D-Bus conn error".to_string()); }
	};

	Ok(Box::new(Network {
		dbus
	}))
}

