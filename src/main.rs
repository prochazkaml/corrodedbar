mod config;
mod module;
mod modules;
mod scheduler;
use x11::xlib;

fn setrootname(name: &str) {
	unsafe {
		let c_str = std::ffi::CString::new(name).unwrap();

		let dpy = xlib::XOpenDisplay(std::ptr::null());

		let screen = xlib::XDefaultScreen(dpy);
		let rootwin = xlib::XRootWindow(dpy, screen);

		xlib::XStoreName(dpy, rootwin, c_str.as_ptr());

		xlib::XCloseDisplay(dpy);
	}
}

fn main() {
	// Load the config file

	let config = match config::loadconfig() {
		Ok(cfg) => cfg,
		Err(errmsg) => {
			setrootname(&errmsg);
			println!("{}", errmsg);
			return;
		}
	};

	// Initialize all modules

	let loadedmodules = match modules::init(&config) {
		Ok(val) => val,
		Err(errmsg) => {
			setrootname(&errmsg);
			println!("{}", errmsg);
			return;
		}
	};

	println!("{} module(s) enabled.", loadedmodules.len());

	// Start the scheduler
	
	scheduler::run(&config, &loadedmodules);
}

