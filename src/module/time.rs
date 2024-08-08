use crate::config;
use crate::modules;
use crate::configoptional;

struct Time {
	format: String
}

impl modules::ModuleImplementation for Time {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		Ok(Some(format!("{}", chrono::Local::now().format(&self.format))))
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Time {
		format: configoptional!(config, "_format", "%H:%M".to_string())
	}))
}


