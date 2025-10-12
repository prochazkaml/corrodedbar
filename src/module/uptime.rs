use crate::modules;
use crate::utils;

use toml::Table;

#[derive(serde::Deserialize)]
struct Uptime {
	#[serde(default = "default_format")]
	format: String
}

fn default_format() -> String { "%dd %Hh %Mm".to_string() }

impl modules::ModuleImplementation for Uptime {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let uptime: f64 = utils::read_line_as("/proc/uptime")?;

		utils::format_duration(&self.format, uptime)
	}
}

pub fn init(config: Table) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let new: Uptime = Table::try_into(config).map_err(|err| format!("Error reading `uptime` config: {err}"))?;

	Ok(Box::new(new))
}

