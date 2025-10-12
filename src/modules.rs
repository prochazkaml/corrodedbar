use crate::config;
use crate::module::{backlight, battery, bluetooth, cpu, memory, microphone, network, time, uptime, volume};
use std::time::Duration;

#[macro_export]
macro_rules! config_mandatory {
	($from:ident, $idx:literal) => {
		config::get_key_value_as($from, $idx).ok_or_else(|| format!("Error: {} not defined in the config", $idx))?
	}
}

#[macro_export]
macro_rules! config_optional {
	($from:ident, $idx:literal, $default:expr) => {
		config::get_key_value_default_as($from, $idx, $default)
	};
}

pub trait ModuleImplementation {
	fn run(&mut self, ts: Duration) -> Result<Option<String>, String>;
}

pub struct ModuleRuntime {
	pub module: Box<dyn ModuleImplementation>,
	pub name: String,
	pub icon: Option<String>,
	pub unixsignal: Option<u8>,
	pub interval: Duration,
	pub startdelay: Duration
}

macro_rules! register_module {
	($name:ident) => {
		(stringify!($name), $name::init)
	};
}

type ModuleInitFun = fn(&Vec<config::ConfigKeyValue>) -> Result<Box<dyn ModuleImplementation>, String>;

pub fn init(config: &Vec<config::ConfigModule>) -> Result<Vec<ModuleRuntime>, String> {
	let available_modules: Vec<(&str, ModuleInitFun)> = vec![
		register_module!(battery),
		register_module!(backlight),
		register_module!(bluetooth),
		register_module!(cpu),
		register_module!(memory),
		register_module!(microphone),
		register_module!(network),
		register_module!(time),
		register_module!(uptime),
		register_module!(volume)
	];

	let mut loaded_modules: Vec<ModuleRuntime> = Vec::new();

	let Some(cfg_modules_str) = config::get_key_value(config::get_module(config, "general").unwrap(), "modules") else {
		Err("There are no modules to load - module [general] must contain a list of modules to enable.".to_string())?
	};

	let enabled_modules: Vec<&str> = cfg_modules_str.split_whitespace().collect();

	for (i, name) in enabled_modules.iter().enumerate() {
		println!("[{}/{}] Initializing module {}", i + 1, enabled_modules.len(), name);

		let Some(mod_settings) = config::get_module(config, name) else {
			println!(" -> module is not configured at all, skipping");
			continue
		};

		let Some(implem) = config::get_key_value(mod_settings, "implements") else {
			println!(" -> module does not contain an \"implements\" param, skipping");
			continue
		};

		let module_init = available_modules.iter()
			.filter(|module| module.0 == implem)
			.map(|module| module.1).next();

		let Some(module_init) = module_init else {
			println!(" -> could not find an implementation for {}, skipping", implem);
			continue
		};

		let interval: u64 = config::get_key_value_default_as(mod_settings, "interval", 1000);
		let start_delay: u64 = config::get_key_value_default_as(mod_settings, "startdelay", 0);

		match module_init(mod_settings) {
			Ok(val) => loaded_modules.push(ModuleRuntime {
				module: val,
				name: implem.to_string(),
				icon: config::get_key_value(mod_settings, "icon").map(|x| x.to_string()),
				unixsignal: config::get_key_value_as(mod_settings, "unixsignal"),
				interval: Duration::from_millis(interval),
				startdelay: Duration::from_millis(start_delay)
			}),
			Err(val) => println!(" -> {}", val)
		}
	}

	Ok(loaded_modules)
}

