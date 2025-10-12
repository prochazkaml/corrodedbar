use crate::utils;

use serde::{Deserialize, Deserializer};
use toml::{Value, Table};
use std::time::{Duration, SystemTime};

#[derive(serde::Deserialize)]
pub struct Config {
	#[serde(default = "default_spaces::<1>")]
	pub left_pad: String,

	#[serde(default = "default_spaces::<1>")]
	pub right_pad: String,

	#[serde(default = "default_spaces::<2>")]
	pub delim: String,

	#[serde(deserialize_with = "deserialize_millis")]
	#[serde(default = "default_max_interval")]
	pub max_interval: Duration,

	#[serde(skip)]
	#[serde(default = "default_mtime")]
	pub mtime: SystemTime,

	pub modules: Vec<ModuleConfig>
}

fn default_spaces<const N: usize>() -> String { " ".repeat(N) }
fn default_max_interval() -> Duration { Duration::MAX }
fn default_mtime() -> SystemTime { SystemTime::now() }

fn deserialize_millis<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
	let val = Value::deserialize(deserializer)?;

	if let Value::Integer(dur) = val {
		return Ok(Duration::from_millis(dur as u64))
	}

	if let Value::Float(dur) = val {
		return Ok(Duration::from_secs_f64(dur * 1000.0))
	}

	Err(serde::de::Error::custom("Expected max_interval to be an integer or float"))
}

#[derive(serde::Deserialize, Clone)]
pub struct ModuleConfig {
	pub icon: Option<String>,

	#[serde(deserialize_with = "deserialize_millis")]
	pub interval: Duration,

	#[serde(deserialize_with = "deserialize_millis")]
	#[serde(default = "default_start_delay")]
	pub start_delay: Duration,

	pub unix_signal: Option<u8>,

	#[serde(deserialize_with = "deserialize_module_impl")]
	#[serde(rename = "impl")]
	pub implementation: ModuleImplementationConfig
}

fn default_start_delay() -> Duration { Duration::ZERO }

#[derive(Clone)]
pub struct ModuleImplementationConfig {
	pub name: String,
	pub config: Table
}

fn deserialize_module_impl<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ModuleImplementationConfig, D::Error> {
	let impl_map = Table::deserialize(deserializer)?;

	let mut keys = impl_map.keys();

	if keys.len() != 1 {
		Err(serde::de::Error::custom("Expected a single key in `impl` table"))?
	}

	let key = keys.next().unwrap();

	let Value::Table(module) = &impl_map[key] else {
		Err(serde::de::Error::custom("Expected a table in `impl` table"))?
	};

	Ok(ModuleImplementationConfig {
		name: key.to_string(),
		config: module.clone()
	})
}

impl Config {
	fn from_toml(config: &str) -> Result<Self, String> {
		let config = toml::from_str(config).map_err(|err| format!("Error parsing TOML: {err}"))?;
		Ok(config)
	}
}

fn get_xdg_config_path() -> Option<String> {
	std::env::var_os("XDG_CONFIG_HOME")?.into_string().ok()
}

fn get_fake_xdg_config_path() -> Option<String> {
	Some(std::env::var_os("XDG_CONFIG_HOME")?.into_string().ok()? + "/.config")
}

fn get_general_config_path() -> Option<String> {
	// Try the official XDG config path, if that fails, fall back to $HOME/.config

	get_xdg_config_path().or_else(get_fake_xdg_config_path)
}

fn get_config_path() -> Option<String> {
	Some(get_general_config_path()? + "/corrodedbar")
}

pub fn get_config_file_mtime() -> Result<SystemTime, String> {
	let Some(config_dir_path) = get_config_path() else {
		Err("Could not determine the config directory. Make sure $HOME is set.".to_string())?
	};

	let config_path = config_dir_path + "/main.toml";

	let metadata = std::fs::metadata(config_path)
		.map_err(|e| format!("Error fetching config file metadata: {}", e))?;

	let mtime = metadata.modified()
		.map_err(|e| format!("Error determining config file mtime: {}", e))?;

	Ok(mtime)
}

pub fn load_config() -> Result<Config, String> {
	let Some(config_dir_path) = get_config_path() else {
		Err("Could not determine the config directory. Make sure $HOME is set.".to_string())?
	};

	let config_path = format!("{}/main.toml", &config_dir_path);

	let config_contents = utils::read_string(&config_path).or_else(|_| {
		if let Err(e) = std::fs::create_dir_all(&config_dir_path) {
			Err(format!("Error creating path {}: {}", config_dir_path, e))?
		}

		let exampleconf = include_str!("example.toml");

		if let Err(e) = std::fs::write(&config_path, exampleconf) {
			Err(format!("Error creating config file {}: {}", config_path, e))?
		}

		Ok::<String, String>(exampleconf.to_string())
	})?;

	let mut config = Config::from_toml(&config_contents)?;

	if let Ok(mtime) = get_config_file_mtime() {
		config.mtime = mtime;
	}

	Ok(config)
}

