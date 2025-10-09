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
		let apps = self.handler.list_applications()
			.map_err(|e| format!("PulseAudio error: {}", e))?;

		if apps.len() <= 0 {
			return Ok(None);
		} else {
			return Ok(Some(self.active.to_string()));
		}
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Microphone {
		active: configoptional!(config, "_active", "active".to_string()),
		handler: SourceController::create()
			.map_err(|e| format!("PulseAudio connection error: {}", e))?
	}))
}

