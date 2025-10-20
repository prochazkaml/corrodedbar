pub mod x11;
pub mod stdout;

pub trait Backend {
	fn output(val: &str);
}

