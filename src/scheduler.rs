use crate::config::{self, Config};
use crate::modules;
use crate::wm;
use crate::args;
use std::time::{Duration, Instant};
use signal_hook::iterator::Signals;

pub fn run(config: &Config, modules: &mut Vec<modules::ModuleRuntime>, params: &args::AppParams) {
	let mut counters: Vec<Duration> = Vec::new();
	let mut interrupts: Vec<bool> = vec![false; modules.len()];
	let mut strings: Vec<Option<String>> = vec![None; modules.len()];
	
	for module in &mut *modules {
		counters.push(module.config.start_delay);
	}

	let start = Instant::now();

	let mut last_output = "".to_string();

	let mut signal_ids: Vec<i32> = Vec::new();

	for module in modules.iter() {
		if let Some(val) = module.config.unix_signal {
			signal_ids.push(val as i32);
		}
	}

	let mut signals = Signals::new(signal_ids).unwrap();

	let old_config_mtime = config::get_config_file_mtime();

	loop {
		// Check if the config file has been modified
		
		if !params.noautoreload
			&& let Ok(old) = old_config_mtime
			&& let Ok(val) = config::get_config_file_mtime()
			&& val != old
		{
			return
		}

		// Run each scheduled module

		for signal in signals.pending() {
			for i in 0..modules.len() {
				if modules[i].config.unix_signal == Some(signal as u8) {
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
				println!("Running module {}.", &modules[i].config.implementation.name);
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
				counters[i] += modules[i].config.interval;
			}
		}

		// Generate the output string
		
		let mut output = config.left_pad.clone();

		for i in 0..strings.len() {
			if let Some(val) = &strings[i] {
				if let Some(val) = &modules[i].config.icon {
					output += val;
					output += " ";
				}

				output += val;

				if i < strings.len() - 1 {
					output += &config.delim;
				}
			}
		}

		output += &config.right_pad;

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

			if sleep > config.max_interval {
				sleep = config.max_interval;
			}

			if params.verbose {
				println!("Going to sleep for {:?}.", sleep);
			}

			std::thread::sleep(sleep);
		}
	}
}

