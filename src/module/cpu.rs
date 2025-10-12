use crate::modules;
use crate::utils;
use crate::formatter;
use crate::fmt_opt;

use toml::Table;

#[derive(serde::Deserialize)]
struct Cpu {
	temp_device: String,

	#[serde(default = "default_format")]
	format: String
}

fn default_format() -> String { "%t°C %F MHz".to_string() }

impl Cpu {
	fn try_get_temp(&self) -> Result<Option<f64>, String> {
		let curr_temp: f64 = utils::read_line_as(&self.temp_device)?;

		Ok(Some(curr_temp))
	}

	fn try_find_temp(&self) -> Result<Option<f64>, String> {
		let dir = std::fs::read_dir("/sys/class/hwmon").map_err(|x| x.to_string())?;

		for hwmon in dir {
			let hwmon = hwmon.unwrap().path();
			let hwmon = std::fs::read_dir(&hwmon).map_err(|x| x.to_string())?;

			for temp in hwmon {
				let path = temp.unwrap().path();
				let path = path.to_str().unwrap();

				if !path.ends_with("_label") { continue }

				let Ok(label) = utils::read_line(path) else { continue };

				if label != self.temp_device { continue }

				let currtemp: f64 = utils::read_line_as(&path.replace("_label", "_input"))?;

				return Ok(Some(currtemp))
			}
		}

		Err(format!("Could not find {}", &self.temp_device))
	}

	fn get_temp(&self) -> Result<Option<f64>, String> {
		let attempt = self.try_get_temp();

		if let Ok(val) = attempt {
			return Ok(val)
		}

		if let Ok(val) = self.try_find_temp() {
			return Ok(val)
		}

		attempt
	}

	fn get_freq(&self, proc_cpu_info: &str, highest: bool) -> Result<Option<f64>, String> {
		let lines = proc_cpu_info.lines();

		let default: f64 = if highest { 0.0 } else { 1000000.0 };

		let mut target: f64 = default;

		for line in lines {
			let split: Vec<&str> = line.split_whitespace().collect();

			if split.len() != 4 { continue }

			if split[0] == "cpu" && split[1] == "MHz" {
				let Ok(freq) = split[3].parse::<f64>() else {
					continue
				};

				if (freq > target && highest) || (freq < target && !highest) {
					target = freq;
				}
			}
		}

		if target == default { return Ok(None) }

		Ok(Some(target))
	}

	fn get_highest_freq(&self, proc_cpu_info: &str) -> Result<Option<f64>, String> {
		self.get_freq(proc_cpu_info, true)
	}

	fn get_lowest_freq(&self, proc_cpu_info: &str) -> Result<Option<f64>, String> {
		self.get_freq(proc_cpu_info, false)
	}
}

impl modules::ModuleImplementation for Cpu {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let proc_cpu_info = utils::read_string("/proc/cpuinfo")?;

		formatter::format(&self.format, |tag| {
			match tag {
				't' => fmt_opt!(f64 self.get_temp(), "[d1000 p1]"),
				'F' => fmt_opt!(f64 self.get_highest_freq(&proc_cpu_info)),
				'f' => fmt_opt!(f64 self.get_lowest_freq(&proc_cpu_info)),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: Table) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let new: Cpu = Table::try_into(config).map_err(|err| format!("Error reading `cpu` config: {err}"))?;

	Ok(Box::new(new))
}

