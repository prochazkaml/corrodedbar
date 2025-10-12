use crate::modules;
use crate::utils;
use crate::formatter;
use crate::fmt_opt;

use toml::Table;

#[derive(serde::Deserialize)]
struct Battery {
	device: String,
	
	#[serde(default = "default_format")]
	format: String,

	#[serde(default = "default_est_time_format")]
	est_time_format: String
}

fn default_format() -> String { "%i %p%% (%w W %e)".to_string() }
fn default_est_time_format() -> String { "%h:%M".to_string() }

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

	fn get_capacity(&self) -> Result<f64, String> {
		utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/capacity", self.device))
			.map(|x| x / 100.0)
	}

	fn get_energy(&self) -> Result<f64, String> {
		let energy = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/energy_now", self.device))
			.map(|x| x / 1000000.0);

		if let Ok(energy) = energy {
			return Ok(energy)
		}

		let voltage = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/voltage_min_design", self.device))
			.map(|x| x / 1000000.0)?;

		let charge = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/charge_now", self.device))
			.map(|x| x / 1000000.0)?;

		Ok(voltage * charge)
	}

	fn get_energy_full(&self) -> Result<f64, String> {
		let max_energy = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/energy_full", self.device))
			.map(|x| x / 1000000.0);

		if let Ok(max_energy) = max_energy {
			return Ok(max_energy)
		}

		let min_voltage = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/voltage_min_design", self.device))
			.map(|x| x / 1000000.0)?;

		let max_charge = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/charge_full", self.device))
			.map(|x| x / 1000000.0)?;

		Ok(min_voltage * max_charge)
	}

	fn get_power(&self) -> Result<f64, String> {
		let power = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/power_now", self.device))
			.map(|x| x / 1000000.0);

		if let Ok(power) = power {
			return Ok(power)
		}

		let min_voltage = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/voltage_now", self.device))
			.map(|x| x / 1000000.0)?;

		let current = utils::read_line_as::<f64>(&format!("/sys/class/power_supply/{}/current_now", self.device))
			.map(|x| x / 1000000.0)?;

		Ok(min_voltage * current)
	}

	fn get_estimate(&self) -> Result<Option<String>, String> {
		let empty = Ok(Some("--:--".to_string()));

		let status = utils::read_line(&format!("/sys/class/power_supply/{}/status", self.device))?;

		let power = self.get_power()?;

		if power == 0.0 {
			return empty;
		}

		let energy_now = self.get_energy()?;
		
		match status.as_str() {
			"Charging" => {
				let energy_full = self.get_energy_full()?;

				utils::format_duration(&self.est_time_format, (energy_full - energy_now) * 3600.0 / power)
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
				'p' => fmt_opt!(f64 self.get_capacity().map(Some), "[d.01]"),
				'w' => fmt_opt!(f64 self.get_power().map(Some), "[p1]"),
				'e' => fmt_opt!(String self.get_estimate()),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: Table) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let new: Battery = Table::try_into(config).map_err(|err| format!("Error reading `time` config: {err}"))?;

	Ok(Box::new(new))
}

