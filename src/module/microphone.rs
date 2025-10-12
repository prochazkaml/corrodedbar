use crate::config;
use crate::modules;
use crate::config_optional;

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

		Ok((!apps.is_empty()).then(|| self.active.to_string()))
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Microphone {
		active: config_optional!(config, "_active", "active".to_string()),
		handler: SourceController::create()
			.map_err(|e| format!("PulseAudio connection error: {}", e))?
	}))
}

