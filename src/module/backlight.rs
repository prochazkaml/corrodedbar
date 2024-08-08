use crate::config;
use crate::modules;
use crate::utils;
use crate::formatter;
use crate::fmtopt;
use crate::configmandatory;
use crate::configoptional;

struct Backlight {
	devicecurr: String,
	devicemax: String,
	format: String
}

impl Backlight {
	fn getvalue(&self) -> Result<Option<i64>, String> {
		let curr: i64 = utils::readlineas(&self.devicecurr)?;

		Ok(Some(curr))
	}

	fn getmaxvalue(&self) -> Result<Option<i64>, String> {
		let max: i64 = utils::readlineas(&self.devicemax)?;

		Ok(Some(max))
	}

	fn getvalueperc(&self) -> Result<Option<f64>, String> {
		let curr: f64 = self.getvalue()?.unwrap() as f64;
		let max: f64 = self.getmaxvalue()?.unwrap() as f64;

		Ok(Some(curr / max))
	}
}

impl modules::ModuleImplementation for Backlight {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		formatter::format(&self.format, |tag| {
			match tag {
				'c' => fmtopt!(i64 self.getvalue()),
				'u' => fmtopt!(f64 self.getvalueperc(), "[d.01]"),
				'm' => fmtopt!(i64 self.getmaxvalue()),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Backlight {
		devicecurr: configmandatory!(config, "_devicecurr"),
		devicemax: configmandatory!(config, "_devicemax"),
		format: configoptional!(config, "_format", "%u%%".to_string())
	}))
}

