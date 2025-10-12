mod config;
mod module;
mod modules;
mod scheduler;
mod wm;
mod utils;
mod args;
mod formatter;

fn run(params: &args::AppParams) -> Result<(), String> {
	let config = config::load_config()?;

	let mut loaded_modules = modules::init(&config)?;
	println!("{} module(s) enabled.", loaded_modules.len());

	scheduler::run(&config, &mut loaded_modules, params);

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
				wm::set_root_name(&err);
				println!("Init failed: {}", err);
				std::thread::sleep(std::time::Duration::from_millis(1000));
			}
		}
	}
}

