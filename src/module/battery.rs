use crate::config;
use crate::modules;
use crate::utils;
use crate::formatter;
use crate::fmt_opt;
use crate::config_mandatory;
use crate::config_optional;

struct Battery {
	device: String,
	format: String,
	est_time_format: String
}

impl Battery {
	fn get_icon(&self) -> Result<Option<String>, String> {
		let status = utils::read_line(&format!("/sys/class/power_supply/{}/status", self.device))?;

		let icon = match status.as_str() {
			"Charging" => "🔌",
			"Full" => "✔️",
			"Not charging" => "✔️",
			"Discharging" => "🔋",
			_ => "?"
		};
		
		Ok(Some(icon.to_string()))
	}

	fn get_percentage(&self) -> Result<Option<f64>, String> {
		let perc: f64 = utils::read_line_as(&format!("/sys/class/power_supply/{}/capacity", self.device))?;

		Ok(Some(perc / 100.0))
	}

	fn get_power(&self) -> Result<Option<f64>, String> {
		let power: f64 = utils::read_line_as(&format!("/sys/class/power_supply/{}/power_now", self.device))?;

		Ok(Some(power / 1000000.0))
	}

	fn get_estimate(&self) -> Result<Option<String>, String> {
		let empty = Ok(Some("--:--".to_string()));

		let status = utils::read_line(&format!("/sys/class/power_supply/{}/status", self.device))?;

		let power: f64 = utils::read_line_as(&format!("/sys/class/power_supply/{}/power_now", self.device))?;

		if power == 0.0 {
			return empty;
		}

		let energy_now: f64 = utils::read_line_as(&format!("/sys/class/power_supply/{}/energy_now", self.device))?;
		
		match status.as_str() {
			"Charging" => {
				let energyfull: f64 = utils::read_line_as(&format!("/sys/class/power_supply/{}/energy_full", self.device))?;

				utils::format_duration(&self.est_time_format, (energyfull - energy_now) * 3600.0 / power)
			},
			"Discharging" => {
				utils::format_duration(&self.est_time_format, energy_now * 3600.0 / power)
			},
			_ => { empty }
		}
	}
}

impl modules::ModuleImplementation for Battery {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		formatter::format(&self.format, |tag| {
			match tag {
				'i' => fmt_opt!(String self.get_icon()),
				'p' => fmt_opt!(f64 self.get_percentage(), "[d.01]"),
				'w' => fmt_opt!(f64 self.get_power(), "[p1]"),
				'e' => fmt_opt!(String self.get_estimate()),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Battery {
		device: config_mandatory!(config, "_device"),
		format: config_optional!(config, "_format", "%i %p%% (%w W %e)".to_string()),
		est_time_format: config_optional!(config, "_esttimeformat", "%h:%M".to_string())
	}))
}

