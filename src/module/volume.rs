use crate::config;
use crate::modules;

use pulsectl::controllers::SinkController;
use pulsectl::controllers::DeviceControl;

struct Volume {
	handler: SinkController
}

impl modules::ModuleImplementation for Volume {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let dev = match self.handler.get_default_device() {
			Ok(val) => val,
			Err(_) => { return Err("Error getting default device".to_string()); }
		};

		Ok(Some(if dev.mute {
			"off".to_string()
		} else {
			dev.volume.get()[0].to_string().trim().to_string()
		}))
	}
}

pub fn init(_config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let handler = match SinkController::create() {
		Ok(val) => val,
		Err(_) => { return Err("PulseAudio conn error".to_string()); }
	};

	Ok(Box::new(Volume {
		handler
	}))
}

