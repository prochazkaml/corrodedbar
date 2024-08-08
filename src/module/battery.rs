use crate::config;
use crate::modules;
use crate::utils;
use crate::formatter;
use crate::fmtopt;
use crate::configmandatory;
use crate::configoptional;

struct Battery {
	device: String,
	format: String,
	esttimeformat: String
}

impl Battery {
	fn geticon(&self) -> Result<Option<String>, String> {
		let icons = std::collections::HashMap::from([
			("Charging".to_string(), "🔌"),
			("Full".to_string(), "✔️"),
			("Discharging".to_string(), "🔋")
		]);

		let status = utils::readline(format!("/sys/class/power_supply/{}/status", self.device))?;
		
		Ok(Some(match icons.get(&status) {
			Some(val) => val,
			None => "?"
		}.to_string()))
	}

	fn getpercentage(&self) -> Result<Option<f64>, String> {
		let perc: f64 = utils::readlineas(format!("/sys/class/power_supply/{}/capacity", self.device))?;

		Ok(Some(perc / 100.0))
	}

	fn getpower(&self) -> Result<Option<f64>, String> {
		let power: f64 = utils::readlineas(format!("/sys/class/power_supply/{}/power_now", self.device))?;

		Ok(Some(power / 1000000.0))
	}

	fn getestimate(&self) -> Result<Option<String>, String> {
		let empty = Ok(Some("--:--".to_string()));

		let status = utils::readline(format!("/sys/class/power_supply/{}/status", self.device))?;

		let power: f64 = utils::readlineas(format!("/sys/class/power_supply/{}/power_now", self.device))?;
				
		if power == 0.0 {
			return empty;
		}

		let energynow: f64 = utils::readlineas(format!("/sys/class/power_supply/{}/energy_now", self.device))?;
		
		match status.as_str() {
			"Charging" => {
				let energyfull: f64 = utils::readlineas(format!("/sys/class/power_supply/{}/energy_full", self.device))?;

				utils::formatduration(&self.esttimeformat, (energyfull - energynow) * 3600.0 / power)
			},
			"Discharging" => {
				utils::formatduration(&self.esttimeformat, energynow * 3600.0 / power)
			},
			_ => { empty }
		}
	}
}

impl modules::ModuleImplementation for Battery {
	fn run(&mut self, ts: std::time::Duration) -> Result<Option<String>, String> {
		formatter::format(&self.format, |tag| {
			match tag {
				'i' => fmtopt!(String self.geticon()),
				'p' => fmtopt!(f64 self.getpercentage(), "[d.01]"),
				'w' => fmtopt!(f64 self.getpower(), "[p1]"),
				'e' => fmtopt!(String self.getestimate()),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Battery {
		device: configmandatory!(config, "_device"),
		format: configoptional!(config, "_format", "%i %p%% (%w W %e)".to_string()),
		esttimeformat: configoptional!(config, "_esttimeformat", "%h:%M".to_string())
	}))
}

