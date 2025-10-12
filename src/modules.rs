use crate::config::{Config, ModuleConfig};
use crate::module::{backlight, battery, bluetooth, cpu, memory, microphone, network, time, uptime, volume};

use itertools::Itertools;
use toml::Table;

use std::time::Duration;

pub trait ModuleImplementation {
	fn run(&mut self, ts: Duration) -> Result<Option<String>, String>;
}

pub struct ModuleRuntime {
	pub module: Box<dyn ModuleImplementation>,
	pub config: ModuleConfig
}

macro_rules! register_module {
	($name:ident) => {
		(stringify!($name), $name::init)
	};
}

type ModuleInitFun = fn(Table) -> Result<Box<dyn ModuleImplementation>, String>;

pub fn init(config: &Config) -> Result<Vec<ModuleRuntime>, String> {
	let available_modules: &[(&str, ModuleInitFun)] = &[
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

	let loaded_modules = config.modules.iter().enumerate()
		.filter_map(|(i, module_config)| {
			println!("[{}/{}] Initializing module {}", i + 1, config.modules.len(), module_config.implementation.name);
			
			let module_init = available_modules.iter()
				.filter(|module| module.0 == module_config.implementation.name)
				.map(|module| module.1).next();

			let Some(module_init) = module_init else {
				println!(" -> does not exist, skipping");
				None?
			};

			module_init(module_config.implementation.config.clone())
				.map_err(|err| println!(" -> {}", err)).ok()
				.map(|module| ModuleRuntime {
					module,
					config: module_config.clone()
				})
		})
		.collect_vec();

	Ok(loaded_modules)
}

