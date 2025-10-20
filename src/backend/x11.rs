use x11::xlib;

use crate::backend::Backend;

pub struct X11Backend {}

impl Backend for X11Backend {
	fn output(val: &str) {
		unsafe {
			let c_str = std::ffi::CString::new(val).unwrap();

			let dpy = xlib::XOpenDisplay(std::ptr::null());

			let screen = xlib::XDefaultScreen(dpy);
			let root_win = xlib::XRootWindow(dpy, screen);

			xlib::XStoreName(dpy, root_win, c_str.as_ptr());

			xlib::XCloseDisplay(dpy);
		}
	}
}

