use crate::modules;

use toml::Table;

#[derive(serde::Deserialize)]
struct Time {
	#[serde(default = "default_format")]
	format: String
}

fn default_format() -> String { "%H:%M".to_string() }

impl modules::ModuleImplementation for Time {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		Ok(Some(chrono::Local::now().format(&self.format).to_string()))
	}
}

pub fn init(config: Table) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let new: Time = Table::try_into(config).map_err(|err| format!("Error reading `time` config: {err}"))?;

	Ok(Box::new(new))
}

