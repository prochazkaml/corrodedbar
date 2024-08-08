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
	fn gettemp(&self) -> Result<Option<f64>, String> {
		let currtemp: f64 = utils::readlineas(&format!("{}", self.tempdevice))?;

		Ok(Some(currtemp))
	}

	fn getfreq(&self, proccpuinfo: String, highest: bool) -> Result<Option<f64>, String> {
		let lines = proccpuinfo.lines();

		let default: f64 = if highest { 0.0 } else { 1000000.0 };

		let mut target: f64 = default;

		for line in lines {
			let split: Vec<&str> = line.split_whitespace().collect();

			if split.len() != 4 { continue; }
			
			if split[0] == "cpu" && split[1] == "MHz" {
				let freq = match split[3].parse::<f64>() {
					Ok(val) => val,
					Err(_) => { continue; }
				};
				
				if (freq > target && highest) || (freq < target && !highest) {
					target = freq;
				}
			}
		}
		
		Ok(if target != default {
			Some(target)
		} else {
			None
		})
	}

	fn gethighestfreq(&self, proccpuinfo: String) -> Result<Option<f64>, String> {
		self.getfreq(proccpuinfo, true)
	}

	fn getlowestfreq(&self, proccpuinfo: String) -> Result<Option<f64>, String> {
		self.getfreq(proccpuinfo, false)
	}
}

impl modules::ModuleImplementation for Cpu {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let proccpuinfo = utils::readstring("/proc/cpuinfo")?;

		formatter::format(&self.format, |tag| {
			match tag {
				't' => fmtopt!(f64 self.gettemp(), "[d1000 p1]"),
				'F' => fmtopt!(f64 self.gethighestfreq(proccpuinfo.clone())),
				'f' => fmtopt!(f64 self.getlowestfreq(proccpuinfo.clone())),
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

