use crate::modules;

use pulsectl::controllers::SourceController;
use pulsectl::controllers::AppControl;
use toml::Table;

#[derive(serde::Deserialize)]
struct MicrophoneConfig {
	#[serde(default = "default_active")]
	active: String
}

fn default_active() -> String { "active".to_string() }

struct Microphone {
	config: MicrophoneConfig,
	handler: SourceController
}

impl modules::ModuleImplementation for Microphone {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let apps = self.handler.list_applications()
			.map_err(|e| format!("PulseAudio error: {}", e))?;

		Ok((!apps.is_empty()).then(|| self.config.active.to_string()))
	}
}

pub fn init(config: Table) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let config: MicrophoneConfig = Table::try_into(config).map_err(|err| format!("Error reading `microphone` config: {err}"))?;

	Ok(Box::new(Microphone {
		config,
		handler: SourceController::create()
			.map_err(|e| format!("PulseAudio connection error: {}", e))?
	}))
}

