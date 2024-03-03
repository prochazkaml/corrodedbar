use x11::xlib;

pub fn setrootname(name: &str) {
	unsafe {
		let c_str = std::ffi::CString::new(name).unwrap();

		let dpy = xlib::XOpenDisplay(std::ptr::null());

		let screen = xlib::XDefaultScreen(dpy);
		let rootwin = xlib::XRootWindow(dpy, screen);

		xlib::XStoreName(dpy, rootwin, c_str.as_ptr());

		xlib::XCloseDisplay(dpy);
	}
}

