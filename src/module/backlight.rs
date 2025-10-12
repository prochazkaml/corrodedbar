use crate::modules;
use crate::utils;
use crate::formatter;
use crate::fmt_opt;

use toml::Table;

#[derive(serde::Deserialize)]
struct Backlight {
	device_curr: String,
	device_max: String,
	
	#[serde(default = "default_format")]
	format: String
}

fn default_format() -> String { "%u%%".to_string() }

impl Backlight {
	fn get_value(&self) -> Result<Option<i64>, String> {
		let curr: i64 = utils::read_line_as(&self.device_curr)?;

		Ok(Some(curr))
	}

	fn get_max_value(&self) -> Result<Option<i64>, String> {
		let max: i64 = utils::read_line_as(&self.device_max)?;

		Ok(Some(max))
	}

	fn get_value_perc(&self) -> Result<Option<f64>, String> {
		let curr: f64 = self.get_value()?.unwrap() as f64;
		let max: f64 = self.get_max_value()?.unwrap() as f64;

		Ok(Some(curr / max))
	}
}

impl modules::ModuleImplementation for Backlight {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		formatter::format(&self.format, |tag| {
			match tag {
				'c' => fmt_opt!(i64 self.get_value()),
				'u' => fmt_opt!(f64 self.get_value_perc(), "[d.01]"),
				'm' => fmt_opt!(i64 self.get_max_value()),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: Table) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let new: Backlight = Table::try_into(config).map_err(|err| format!("Error reading `time` config: {err}"))?;

	Ok(Box::new(new))
}

