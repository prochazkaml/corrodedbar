use x11::xlib;

pub fn set_root_name(name: &str) {
	unsafe {
		let c_str = std::ffi::CString::new(name).unwrap();

		let dpy = xlib::XOpenDisplay(std::ptr::null());

		let screen = xlib::XDefaultScreen(dpy);
		let root_win = xlib::XRootWindow(dpy, screen);

		xlib::XStoreName(dpy, root_win, c_str.as_ptr());

		xlib::XCloseDisplay(dpy);
	}
}

