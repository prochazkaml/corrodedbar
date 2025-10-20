#[derive(argp::FromArgs)]
#[argp(description = "corrodedbar - a simple X11 statusbar")]
pub struct AppParams {
	#[argp(option, default = "\"x11\".to_string()")]
	#[argp(description = "Selects the backend. Available options: \"x11\", \"stdout\".")]
	pub backend: String,

	#[argp(switch, short = 'v')]
	#[argp(description = "Enable verbose logging during runtime.")]
	pub verbose: bool,

	#[argp(switch, short = 'n')]
	#[argp(description = "Disable autoreload if the config file changes.")]
	pub noautoreload: bool
}

pub fn init() -> AppParams {
	argp::parse_args_or_exit(argp::DEFAULT)
}

