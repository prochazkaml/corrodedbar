use crate::config;
use crate::modules;
use crate::configoptional;

use pulsectl::controllers::SourceController;
use pulsectl::controllers::AppControl;

struct Microphone {
	active: String,
	handler: SourceController
}

impl modules::ModuleImplementation for Microphone {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let apps = match self.handler.list_applications() {
			Ok(val) => val,
			Err(_) => { return Err("PulseAudio error".to_string()); }
		};

		if apps.len() <= 0 {
			return Ok(None);
		} else {
			return Ok(Some(self.active.clone()));
		}
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Microphone {
		active: configoptional!(config, "_active", "active".to_string()),
		handler: match SourceController::create() {
			Ok(val) => val,
			Err(_) => { return Err("PulseAudio connection error".to_string()) }
		}
	}))
}

