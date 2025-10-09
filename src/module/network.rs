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

fn get_addrlist<T: Any>(dev: T) -> Option<Vec<Vec<u32>>> {
	let Ok(cfg) = dev.ip4_config() else { None? };
	
	let Ok(addrlist) = cfg.addresses() else { None? };

	Some(addrlist)
}

impl modules::ModuleImplementation for Network {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let mut ips: Vec<String> = Vec::new();

		let nm = NetworkManager::new(&self.dbus);

		let Ok(devices) = nm.get_devices() else {
			return Ok(None)
		};

		for device in devices {
			let deviplist = match device {
				Device::WiFi(wifi) => get_addrlist(wifi),
				Device::Ethernet(eth) => get_addrlist(eth),
				_ => None // TODO - other interfaces, IPv6
			};

			let Some(deviplist) = deviplist else {
				continue
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

		return Ok(Some(output))
	}
}

pub fn init(_config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	// TODO - specific connection
	
	let dbus = dbus::blocking::Connection::new_system()
		.map_err(|e| format!("D-Bus conn error: {}", e))?;

	Ok(Box::new(Network {
		dbus
	}))
}

