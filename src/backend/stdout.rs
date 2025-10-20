use crate::backend::Backend;

pub struct StdoutBackend {}

impl Backend for StdoutBackend {
	fn output(val: &str) {
		println!("{}", val); // As simple as it gets.
	}
}

