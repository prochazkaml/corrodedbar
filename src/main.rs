mod config;
mod module;
mod modules;
mod scheduler;
mod wm;

fn main() {
	// Load the config file

	let config = match config::loadconfig() {
		Ok(cfg) => cfg,
		Err(errmsg) => {
			wm::setrootname(&errmsg);
			println!("{}", errmsg);
			return;
		}
	};

	// Initialize all modules

	let loadedmodules = match modules::init(&config) {
		Ok(val) => val,
		Err(errmsg) => {
			wm::setrootname(&errmsg);
			println!("{}", errmsg);
			return;
		}
	};

	println!("{} module(s) enabled.", loadedmodules.len());

	// Start the scheduler
	
	scheduler::run(&config, &loadedmodules);
}

