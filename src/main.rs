mod config;
mod module;
mod modules;
mod scheduler;
mod wm;
mod utils;

// TODO - make config autoreload disableable with commandline params

fn run() -> bool {
	// Load the config file

	let config = match config::loadconfig() {
		Ok(cfg) => cfg,
		Err(errmsg) => {
			wm::setrootname(&errmsg);
			println!("{}", errmsg);
			return false;
		}
	};

	// Initialize all modules

	let loadedmodules = match modules::init(&config) {
		Ok(val) => val,
		Err(errmsg) => {
			wm::setrootname(&errmsg);
			println!("{}", errmsg);
			return false;
		}
	};

	println!("{} module(s) enabled.", loadedmodules.len());

	// Start the scheduler
	
	scheduler::run(&config, &loadedmodules);

    true
}

fn main() {
    loop {
        match run() {
            true => {
                println!("Detected config file change, reloading.");
            },
            false => {
                println!("Init failed. Will try again in 1s...");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }
    }
}

