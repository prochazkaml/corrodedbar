use crate::config;
use crate::modules;
use crate::formatter;
use crate::fmtopt;
use crate::configoptional;
use crate::utils;

struct Memory {
	format: String
}

fn getmeminfo(total: f64, free: f64, percentage: bool, calculateused: bool) -> Result<Option<f64>, String> {
	if free < 0.0 || total <= 0.0 { return Ok(None) }

	let val = match calculateused {
		true => total - free,
		false => free
	};

	let val = match percentage {
		true => val / total,
		false => val
	};

	Ok(Some(val))
}

impl modules::ModuleImplementation for Memory {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let file = utils::readstring("/proc/meminfo")?;

		let lines = file.lines();

		let mut total: f64 = -1.0;
		let mut free: f64 = -1.0;

		let mut swaptotal: f64 = -1.0;
		let mut swapfree: f64 = -1.0;

		for line in lines {
			let split: Vec<&str> = line.split_whitespace().collect();

			if split.len() != 3 { continue }

			if split[0] == "MemTotal:" {
				total = split[1].parse::<f64>()
					.map_err(|e| format!("Format error at MemTotal: {}", e))?
			}

			if split[0] == "MemAvailable:" {
				free = split[1].parse::<f64>()
					.map_err(|e| format!("Format error at MemAvailable: {}", e))?
			}

			if split[0] == "SwapTotal:" {
				swaptotal = split[1].parse::<f64>()
					.map_err(|e| format!("Format error at SwapTotal: {}", e))?
			}

			if split[0] == "SwapFree:" {
				swapfree = split[1].parse::<f64>()
					.map_err(|e| format!("Format error at SwapFree: {}", e))?
			}
		}

		formatter::format(&self.format, |tag| {
			match tag {
				'p' => fmtopt!(f64 getmeminfo(total, free, true, true), "[d.01]"),
				'P' => fmtopt!(f64 getmeminfo(total, free, true, false), "[d.01]"),
				'h' => fmtopt!(f64 getmeminfo(total, free, false, true)),
				'H' => fmtopt!(f64 getmeminfo(total, free, false, false)),
				's' => fmtopt!(f64 getmeminfo(swaptotal, swapfree, true, true), "[d.01]"),
				'S' => fmtopt!(f64 getmeminfo(swaptotal, swapfree, true, false), "[d.01]"),
				'w' => fmtopt!(f64 getmeminfo(swaptotal, swapfree, false, true)),
				'W' => fmtopt!(f64 getmeminfo(swaptotal, swapfree, false, false)),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Memory {
		format: configoptional!(config, "_format", "%p%%/%s%%".to_string())
	}))
}

