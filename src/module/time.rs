use crate::config;
use crate::modules;
use crate::config_optional;

struct Time {
	format: String
}

impl modules::ModuleImplementation for Time {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		Ok(Some(chrono::Local::now().format(&self.format).to_string()))
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Time {
		format: config_optional!(config, "_format", "%H:%M".to_string())
	}))
}


