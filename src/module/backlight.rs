use crate::config;
use crate::modules;
use crate::utils;
use crate::formatter;
use crate::fmt_opt;
use crate::config_mandatory;
use crate::config_optional;

struct Backlight {
	device_curr: String,
	device_max: String,
	format: String
}

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

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Backlight {
		device_curr: config_mandatory!(config, "_devicecurr"),
		device_max: config_mandatory!(config, "_devicemax"),
		format: config_optional!(config, "_format", "%u%%".to_string())
	}))
}

