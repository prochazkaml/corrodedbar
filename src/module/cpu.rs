use crate::config;
use crate::modules;
use crate::utils;
use crate::formatter;
use crate::fmtopt;
use crate::configmandatory;
use crate::configoptional;

struct Cpu {
	tempdevice: String,
	format: String
}

impl Cpu {
	fn trygettemp(&self) -> Result<Option<f64>, String> {
		let currtemp: f64 = utils::readlineas(&self.tempdevice)?;

		Ok(Some(currtemp))
	}

	fn tryfindtemp(&self) -> Result<Option<f64>, String> {
		let dir = std::fs::read_dir("/sys/class/hwmon").map_err(|x| x.to_string())?;

		for hwmon in dir {
			let hwmon = hwmon.unwrap().path();
			let hwmon = std::fs::read_dir(&hwmon).map_err(|x| x.to_string())?;

			for temp in hwmon {
				let path = temp.unwrap().path();
				let path = path.to_str().unwrap();

				if !path.ends_with("_label") { continue }

				let Ok(label) = utils::readline(path) else { continue };

				if label != self.tempdevice { continue }

				let currtemp: f64 = utils::readlineas(&path.replace("_label", "_input"))?;

				return Ok(Some(currtemp))
			}
		}

		Err(format!("Could not find {}", &self.tempdevice))
	}

	fn gettemp(&self) -> Result<Option<f64>, String> {
		let attempt = self.trygettemp();

		if let Ok(val) = attempt {
			return Ok(val)
		}

		if let Ok(val) = self.tryfindtemp() {
			return Ok(val)
		}

		attempt
	}

	fn getfreq(&self, proccpuinfo: &str, highest: bool) -> Result<Option<f64>, String> {
		let lines = proccpuinfo.lines();

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

	fn gethighestfreq(&self, proccpuinfo: &str) -> Result<Option<f64>, String> {
		self.getfreq(proccpuinfo, true)
	}

	fn getlowestfreq(&self, proccpuinfo: &str) -> Result<Option<f64>, String> {
		self.getfreq(proccpuinfo, false)
	}
}

impl modules::ModuleImplementation for Cpu {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let proccpuinfo = utils::readstring("/proc/cpuinfo")?;

		formatter::format(&self.format, |tag| {
			match tag {
				't' => fmtopt!(f64 self.gettemp(), "[d1000 p1]"),
				'F' => fmtopt!(f64 self.gethighestfreq(&proccpuinfo)),
				'f' => fmtopt!(f64 self.getlowestfreq(&proccpuinfo)),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Cpu {
		tempdevice: configmandatory!(config, "_tempdevice"),
		format: configoptional!(config, "_format", "%t°C %F MHz".to_string())
	}))
}

