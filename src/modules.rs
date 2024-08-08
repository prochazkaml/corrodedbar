use crate::config;
//use crate::module::{network, bluetooth, memory, uptime, cpu, backlight, microphone, volume, battery, time};
use crate::module::{backlight, battery, bluetooth, cpu};
use std::time::Duration;

#[macro_export]
macro_rules! configmandatory {
    ($from:ident, $idx:literal) => {
        match config::getkeyvalueas($from, $idx) {
            Some(val) => val,
            None => {
                return Err(format!("Error: {} not defined in the config", $idx));
            }
        }
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
	// registermodule!(availablemodules, network);
	// registermodule!(availablemodules, memory);
	// registermodule!(availablemodules, uptime);
	// registermodule!(availablemodules, microphone);
	// registermodule!(availablemodules, volume);
	// registermodule!(availablemodules, time);

	let mut loadedmodules: Vec<ModuleRuntime> = Vec::new();

    let cfgmodulesstr = match config::getkeyvalue(config::getmodule(config, "general").unwrap(), "modules") {
		Some(val) => val,
		None => {
			return Err("There are no modules to load - module [general] must contain a list of modules to enable.".to_string());
		}
	};

    let enabledmodules: Vec<&str> = cfgmodulesstr.split_whitespace().collect();

	for (i, name) in enabledmodules.iter().enumerate() {
		println!("[{}/{}] Initializing module {}", i + 1, enabledmodules.len(), name);

		let modsettings = match config::getmodule(&config, name) {
			Some(val) => val,
			None => {
				println!(" -> module is not configured at all, skipping");
				continue;
			}
		};

		let implem = match config::getkeyvalue(&modsettings, "implements") {
			Some(val) => val,
			None => {
				println!(" -> module does not contain an \"implements\" param, skipping");
				continue;
			}
		};

		let mut moduleinit: Option<&ModuleInitFun> = None;

		for j in 0..availablemodules.len() {
			if implem == availablemodules[j].0 {
				moduleinit = Some(&availablemodules[j].1);
			}
		}

		let moduleinit = match moduleinit {
			Some(val) => val,
			None => {
				println!(" -> could not find an implementation for {}, skipping", implem);
				continue;
			}
		};

		let interval: u64 = config::getkeyvaluedefaultas(&modsettings, "interval", 1000);
		let startdelay: u64 = config::getkeyvaluedefaultas(&modsettings, "startdelay", 0);

		match moduleinit(&modsettings) {
			Ok(val) => loadedmodules.push(ModuleRuntime {
				module: val,
				name: implem,
				icon: config::getkeyvalue(&modsettings, "icon"),
				unixsignal: config::getkeyvalueas(&modsettings, "unixsignal"),
				interval: Duration::from_millis(interval),
				startdelay: Duration::from_millis(startdelay)
			}),
			Err(val) => println!(" -> {}", val)
		}
	}

	return Ok(loadedmodules);
}

pub fn internalerrormsg() -> String {
    return "Internal error".to_string();
}

