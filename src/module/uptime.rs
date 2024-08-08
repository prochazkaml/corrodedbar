use crate::config;
use crate::modules;
use crate::utils;
use crate::configoptional;

struct Uptime {
	format: String
}

impl modules::ModuleImplementation for Uptime {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let uptime: f64 = utils::readlineas("/proc/uptime")?;

		utils::formatduration(&self.format, uptime)
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Uptime {
		format: configoptional!(config, "_format", "%dd %Hh %Mm".to_string())
	}))
}

