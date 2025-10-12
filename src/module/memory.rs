use crate::config;
use crate::modules;
use crate::formatter;
use crate::fmt_opt;
use crate::config_optional;
use crate::utils;

struct Memory {
	format: String
}

fn calculate_value(total: f64, free: f64, percentage: bool, used: bool) -> Result<Option<f64>, String> {
	if free < 0.0 || total <= 0.0 { return Ok(None) }

	let val = match used {
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
		let file = utils::read_string("/proc/meminfo")?;

		let lines = file.lines();

		let mut total: f64 = -1.0;
		let mut free: f64 = -1.0;

		let mut swap_total: f64 = -1.0;
		let mut swap_free: f64 = -1.0;

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
				swap_total = split[1].parse::<f64>()
					.map_err(|e| format!("Format error at SwapTotal: {}", e))?
			}

			if split[0] == "SwapFree:" {
				swap_free = split[1].parse::<f64>()
					.map_err(|e| format!("Format error at SwapFree: {}", e))?
			}
		}

		formatter::format(&self.format, |tag| {
			match tag {
				'p' => fmt_opt!(f64 calculate_value(total, free, true, true), "[d.01]"),
				'P' => fmt_opt!(f64 calculate_value(total, free, true, false), "[d.01]"),
				'h' => fmt_opt!(f64 calculate_value(total, free, false, true)),
				'H' => fmt_opt!(f64 calculate_value(total, free, false, false)),
				's' => fmt_opt!(f64 calculate_value(swap_total, swap_free, true, true), "[d.01]"),
				'S' => fmt_opt!(f64 calculate_value(swap_total, swap_free, true, false), "[d.01]"),
				'w' => fmt_opt!(f64 calculate_value(swap_total, swap_free, false, true)),
				'W' => fmt_opt!(f64 calculate_value(swap_total, swap_free, false, false)),
				_ => Ok(None)
			}
		})
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Memory {
		format: config_optional!(config, "_format", "%p%%/%s%%".to_string())
	}))
}

