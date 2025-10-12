use crate::config;
use crate::modules;
use crate::wm;
use crate::args;
use std::time::{Duration, Instant};
use signal_hook::iterator::Signals;

pub fn run(config: &Vec<config::ConfigModule>, modules: &mut Vec<modules::ModuleRuntime>, params: &args::AppParams) {
	let mut counters: Vec<Duration> = Vec::new();
	let mut interrupts: Vec<bool> = vec![false; modules.len()];
	let mut strings: Vec<Option<String>> = vec![None; modules.len()];
	
	for module in &mut *modules {
		counters.push(module.startdelay);
	}

	let start = Instant::now();

	let general = config::get_module(config, "general").unwrap();

	let defaults: Vec<&'static str> = vec![" ", " ", "  "];

	let left_pad = config::get_key_value_default(general, "leftpad", defaults[0]);
	let right_pad = config::get_key_value_default(general, "rightpad", defaults[1]);
	let delim = config::get_key_value_default(general, "delim", defaults[2]);

	let mut last_output = "".to_string();

	let max_delay: Duration = match config::get_key_value_as(general, "maxinterval") as Option<u64> {
		Some(val) => Duration::from_millis(val),
		None => Duration::MAX
	};

	let mut signal_ids: Vec<i32> = Vec::new();

	for module in modules.iter() {
		if let Some(val) = module.unixsignal {
			signal_ids.push(val as i32);
		}
	}

	let mut signals = Signals::new(signal_ids).unwrap();

	let old_config_mtime = config::get_key_value_default(general, "configmtime", "");

	loop {
		// Check if the config file has been modified
		
		if !params.noautoreload && !old_config_mtime.is_empty() {
			if let Ok(val) = config::get_config_file_mtime() {
				if val != old_config_mtime { return }
			}
		}

		// Run each scheduled module

		for signal in signals.pending() {
			for i in 0..modules.len() {
				if modules[i].unixsignal == Some(signal as u8) {
					interrupts[i] = true;
				}
			}

			if params.verbose {
				println!("Received signal {}.", signal);
			}
		}

		let mut elapsed = start.elapsed();

		for i in 0..modules.len() {
			if elapsed < counters[i] && !interrupts[i] { continue }

			if params.verbose {
				println!("Running module {}.", &modules[i].name);
			}

			strings[i] = match modules[i].module.run(counters[i]) {
				Ok(val) => val,
				Err(err) => {
					if params.verbose {
						println!(" -> {}", err);
					}
					Some(err)
				}
			};

			if interrupts[i] {
				interrupts[i] = false;
			} else {
				counters[i] += modules[i].interval;
			}
		}

		// Generate the output string
		
		let mut output = left_pad.to_string();

		for i in 0..strings.len() {
			if let Some(val) = &strings[i] {
				if let Some(val) = &modules[i].icon {
					output += val;
					output += " ";
				}

				output += val;

				if i < strings.len() - 1 {
					output += delim;
				}
			}
		}

		output += right_pad;

		if output != last_output {
			wm::set_root_name(&output);
			last_output = output;
		}

		// Figure out how much we have to sleep for
		
		let mut least_sleep = Duration::MAX;

		for counter in &counters {
			if *counter < least_sleep {
				least_sleep = *counter;
			}
		}

		elapsed = start.elapsed();

		if least_sleep > elapsed {
			let mut sleep = least_sleep - elapsed;

			if sleep > max_delay {
				sleep = max_delay;
			}

			if params.verbose {
				println!("Going to sleep for {:?}.", sleep);
			}

			std::thread::sleep(sleep);
		}
	}
}

