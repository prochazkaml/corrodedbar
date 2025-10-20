mod config;
mod module;
mod modules;
mod scheduler;
mod backend;
mod utils;
mod args;
mod formatter;

use crate::backend::Backend;
use crate::backend::x11::X11Backend;
use crate::backend::stdout::StdoutBackend;

fn run<B: Backend>(params: &args::AppParams) -> Result<(), String> {
	let config = config::load_config()?;

	let mut loaded_modules = modules::init(&config)?;
	eprintln!("{} module(s) enabled.", loaded_modules.len());

	scheduler::run::<B>(&config, &mut loaded_modules, params);

	Ok(())
}

fn main_with_backend<B: Backend>(params: &args::AppParams) {
	loop {
		match run::<B>(params) {
			Ok(()) => {
				eprintln!("Detected config file change, reloading.");
			},
			Err(err) => {
				B::output(&err);
				eprintln!("Init failed: {}", err);
				std::thread::sleep(std::time::Duration::from_millis(1000));
			}
		}
	}
}

fn main() {
	let params = args::init();

	match params.backend.as_str() {
		"x11" => main_with_backend::<X11Backend>(&params),
		"stdout" => main_with_backend::<StdoutBackend>(&params),
		x => {
			eprintln!("Invalid backend: {}", x);
			std::process::exit(1)
		}
	}
}

