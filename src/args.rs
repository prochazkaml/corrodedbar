#[derive(argp::FromArgs)]
#[argp(description = "corrodedbar - a simple X11 statusbar")]
pub struct AppParams {
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

