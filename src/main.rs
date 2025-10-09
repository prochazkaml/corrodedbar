mod config;
mod module;
mod modules;
mod scheduler;
mod wm;
mod utils;
mod args;
mod formatter;

fn run(params: &args::AppParams) -> Result<(), String> {
	let config = config::loadconfig()?;

	let mut loadedmodules = modules::init(&config)?;
	println!("{} module(s) enabled.", loadedmodules.len());

	scheduler::run(&config, &mut loadedmodules, params);

	Ok(())
}

fn main() {
	let params = args::init();

	loop {
		match run(&params) {
			Ok(()) => {
				println!("Detected config file change, reloading.");
			},
			Err(err) => {
				wm::setrootname(&err);
				println!("Init failed: {}", err);
				std::thread::sleep(std::time::Duration::from_millis(1000));
			}
		}
	}
}

