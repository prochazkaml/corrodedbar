use crate::config;
use crate::module::{backlight, battery, bluetooth, cpu, memory, microphone, network, time, uptime, volume};
use std::time::Duration;

#[macro_export]
macro_rules! configmandatory {
	($from:ident, $idx:literal) => {
		config::getkeyvalueas($from, $idx).ok_or_else(|| format!("Error: {} not defined in the config", $idx))?
	}
}

#[macro_export]
macro_rules! configoptional {
	($from:ident, $idx:literal, $default:expr) => {
		config::getkeyvaluedefaultas($from, $idx, $default)
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

macro_rules! registermodule {
	($dest:ident, $name:ident) => {
		$dest.push((stringify!($name), $name::init));
	};
}

type ModuleInitFun = fn(&Vec<config::ConfigKeyValue>) -> Result<Box<dyn ModuleImplementation>, String>;

pub fn init(config: &Vec<config::ConfigModule>) -> Result<Vec<ModuleRuntime>, String> {
	let mut availablemodules: Vec<(&str, ModuleInitFun)> = Vec::new();

	registermodule!(availablemodules, battery);
	registermodule!(availablemodules, backlight);
	registermodule!(availablemodules, bluetooth);
	registermodule!(availablemodules, cpu);
	registermodule!(availablemodules, memory);
	registermodule!(availablemodules, microphone);
	registermodule!(availablemodules, network);
	registermodule!(availablemodules, time);
	registermodule!(availablemodules, uptime);
	registermodule!(availablemodules, volume);

	let mut loadedmodules: Vec<ModuleRuntime> = Vec::new();

	let Some(cfgmodulesstr) = config::getkeyvalue(config::getmodule(config, "general").unwrap(), "modules") else {
		Err("There are no modules to load - module [general] must contain a list of modules to enable.".to_string())?
	};

	let enabledmodules: Vec<&str> = cfgmodulesstr.split_whitespace().collect();

	for (i, name) in enabledmodules.iter().enumerate() {
		println!("[{}/{}] Initializing module {}", i + 1, enabledmodules.len(), name);

		let Some(modsettings) = config::getmodule(&config, name) else {
			println!(" -> module is not configured at all, skipping");
			continue
		};

		let Some(implem) = config::getkeyvalue(&modsettings, "implements") else {
			println!(" -> module does not contain an \"implements\" param, skipping");
			continue
		};

		let mut moduleinit: Option<&ModuleInitFun> = None;

		for j in 0..availablemodules.len() {
			if implem == availablemodules[j].0 {
				moduleinit = Some(&availablemodules[j].1);
			}
		}

		let Some(moduleinit) = moduleinit else {
			println!(" -> could not find an implementation for {}, skipping", implem);
			continue
		};

		let interval: u64 = config::getkeyvaluedefaultas(&modsettings, "interval", 1000);
		let startdelay: u64 = config::getkeyvaluedefaultas(&modsettings, "startdelay", 0);

		match moduleinit(&modsettings) {
			Ok(val) => loadedmodules.push(ModuleRuntime {
				module: val,
				name: implem.to_string(),
				icon: config::getkeyvalue(&modsettings, "icon").map(|x| x.to_string()),
				unixsignal: config::getkeyvalueas(&modsettings, "unixsignal"),
				interval: Duration::from_millis(interval),
				startdelay: Duration::from_millis(startdelay)
			}),
			Err(val) => println!(" -> {}", val)
		}
	}

	Ok(loadedmodules)
}

pub fn internalerrormsg() -> String {
	"Internal error".to_string()
}

