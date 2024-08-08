use crate::config;
use crate::modules;
use crate::formatter;
use crate::fmtopt;
use crate::configoptional;

struct Memory {
	format: String
}

fn getmeminfo(total: f64, free: f64, percentage: bool, calculateused: bool) -> Result<Option<f64>, String> {
	if free >= 0.0 && total > 0.0 {
		if calculateused {
			if percentage {
				Ok(Some((total - free) / total))
			} else {
				Ok(Some(total - free))
			}
		} else {
			if percentage {
				Ok(Some(free / total))
			} else {
				Ok(Some(free))
			}
		}
	} else {
		Ok(None)
	}
}

impl modules::ModuleImplementation for Memory {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let file = match std::fs::read_to_string("/proc/meminfo") {
			Ok(val) => val,
			Err(errmsg) => { return Err(format!("File read error: {}", errmsg)); }
		};

		let lines = file.lines();

		let mut total: f64 = -1.0;
		let mut free: f64 = -1.0;

		let mut swaptotal: f64 = -1.0;
		let mut swapfree: f64 = -1.0;

		for line in lines {
			let split: Vec<&str> = line.split_whitespace().collect();

			if split.len() != 3 { continue; }

			if split[0] == "MemTotal:" {
				total = match split[1].parse::<f64>() {
					Ok(val) => val,
					Err(_) => { return Err("Format error".to_string()); }
				}
			}

			if split[0] == "MemAvailable:" {
				free = match split[1].parse::<f64>() {
					Ok(val) => val,
					Err(_) => { return Err("Format error".to_string()); }
				}
			}

			if split[0] == "SwapTotal:" {
				swaptotal = match split[1].parse::<f64>() {
					Ok(val) => val,
					Err(_) => { return Err("Format error".to_string()); }
				}
			}

			if split[0] == "SwapFree:" {
				swapfree = match split[1].parse::<f64>() {
					Ok(val) => val,
					Err(_) => { return Err("Format error".to_string()); }
				}
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

