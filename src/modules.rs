use crate::config;
use crate::module::{uptime, backlight, volume, battery, time};
use std::time::Duration;

pub enum ModuleData {
	TypeString(String),
	TypeInt32(i32)
}

type ModuleInitFun = fn(&Vec<config::ConfigKeyValue>) -> Result<Vec<ModuleData>, String>;
type ModuleRunFun = fn(&Vec<ModuleData>, Duration) -> Result<Option<String>, String>;

macro_rules! registermodule {
	($name:ident) => {
		ModuleImplementation {
			name: stringify!($name),
			init: $name::init,
			run: $name::run
		}
	}
}

pub struct ModuleImplementation {
	pub name: &'static str,
	pub init: ModuleInitFun,
	pub run: ModuleRunFun
}

pub static MODULELIST: &[ModuleImplementation] = &[
	registermodule!(uptime),
	registermodule!(backlight),
	registermodule!(volume),
	registermodule!(battery),
	registermodule!(time)
];

pub struct ModuleRuntime<'a> {
	//pub settings: &'a Vec<config::ConfigKeyValue>,
	pub module: &'a ModuleImplementation,
	pub data: Vec<ModuleData>,
	pub icon: Option<&'a String>,
	pub unixsignal: Option<u8>,
	pub interval: Duration,
	pub startdelay: Duration
}

pub fn init(config: &Vec<config::ConfigModule>) -> Result<Vec<ModuleRuntime>, String> {
	let mut loadedmodules: Vec<ModuleRuntime> = Vec::new();

	let enabledmodules: Vec<&str> = match config::getkeyvalue(config::getmodule(config, "general").unwrap(), "modules") {
		Some(val) => val.split_whitespace().collect(),
		None => {
			return Err("There are no modules to load - module [general] must contain a list of modules to enable.".to_string());
		}
	};

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

		let mut module: Option<&ModuleImplementation> = None;

		for j in 0..MODULELIST.len() {
			if implem == MODULELIST[j].name {
				module = Some(&MODULELIST[j]);
			}
		}

		if module.is_none() {
			println!(" -> could not find an implementation for {}, skipping", implem);
			continue;
		}

		let interval: u64 = config::getkeyvaluedefaultas(&modsettings, "interval", 1000);
		let startdelay: u64 = config::getkeyvaluedefaultas(&modsettings, "startdelay", 0);

		match (module.unwrap().init)(&modsettings) {
			Ok(val) => loadedmodules.push(ModuleRuntime {
				//settings: &config[i + 1].settings,
				module: module.unwrap(),
				data: val,
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

