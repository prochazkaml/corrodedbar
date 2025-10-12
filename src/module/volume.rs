use crate::modules;

use pulsectl::controllers::SinkController;
use pulsectl::controllers::DeviceControl;
use toml::Table;

struct Volume {
	handler: SinkController
}

impl modules::ModuleImplementation for Volume {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let dev = self.handler.get_default_device()
			.map_err(|e| format!("Error getting default device: {}", e))?;

		let val = match dev.mute {
			true => "off".to_string(),
			false => dev.volume.get()[0].to_string().trim().to_string()
		};

		Ok(Some(val))
	}
}

pub fn init(_config: Table) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let handler = SinkController::create()
		.map_err(|e| format!("PulseAudio conn error: {}", e))?;

	Ok(Box::new(Volume {
		handler
	}))
}

