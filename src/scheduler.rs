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

	let general = config::getmodule(config, "general").unwrap();

	let defaults: Vec<&'static str> = vec![" ", " ", "  "];

	let leftpad = config::getkeyvaluedefault(general, "leftpad", defaults[0]);
	let rightpad = config::getkeyvaluedefault(general, "rightpad", defaults[1]);
	let delim = config::getkeyvaluedefault(general, "delim", defaults[2]);

	let mut lastoutput = "".to_string();

	let maxdelay: Duration = match config::getkeyvalueas(general, "maxinterval") as Option<u64> {
		Some(val) => Duration::from_millis(val),
		None => Duration::MAX
	};

	let mut signalids: Vec<i32> = Vec::new();

	for module in modules.iter() {
		if let Some(val) = module.unixsignal {
			signalids.push(val as i32);
		}
	}

	let mut signals = Signals::new(signalids).unwrap();

	let oldconfigmtime = config::getkeyvaluedefault(general, "configmtime", "");

	loop {
		// Check if the config file has been modified
		
		if !params.noautoreload && !oldconfigmtime.is_empty() {
			if let Ok(val) = config::getconfigfilemtime() {
				if val != oldconfigmtime { return }
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
		
		let mut output = leftpad.to_string();

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

		output += rightpad;

		if output != lastoutput {
			wm::setrootname(&output);
			lastoutput = output;
		}

		// Figure out how much we have to sleep for
		
		let mut leastsleep = Duration::MAX;

		for counter in &counters {
			if *counter < leastsleep {
				leastsleep = *counter;
			}
		}

		elapsed = start.elapsed();

		if leastsleep > elapsed {
			let mut sleep = leastsleep - elapsed;

			if sleep > maxdelay {
				sleep = maxdelay;
			}

			if params.verbose {
				println!("Going to sleep for {:?}.", sleep);
			}

			std::thread::sleep(sleep);
		}
	}
}

