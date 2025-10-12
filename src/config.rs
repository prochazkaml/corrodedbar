use crate::utils;

pub struct ConfigKeyValue {
	pub key: String,
	pub value: String
}

pub struct ConfigModule {
	pub name: String,
	pub settings: Vec<ConfigKeyValue>
}

pub fn get_module<'a>(cfg: &'a Vec<ConfigModule>, name: &str) -> Option<&'a Vec<ConfigKeyValue>> {
	for module in cfg {
		if module.name == name { return Some(&module.settings); }
	}

	None
}

pub fn get_key_value<'a>(module: &'a Vec<ConfigKeyValue>, key: &str) -> Option<&'a str> {
	for keyvalue in module {
		if keyvalue.key == key { return Some(&keyvalue.value); }
	}
	
	None
}

pub fn get_key_value_default<'a>(module: &'a Vec<ConfigKeyValue>, key: &str, default: &'a str) -> &'a str {
	get_key_value(module, key).unwrap_or(default)
}

pub fn get_key_value_as<T>(module: &Vec<ConfigKeyValue>, key: &str) -> Option<T>
	where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {

	Some(get_key_value(module, key)?.parse::<T>().unwrap())
}

pub fn get_key_value_default_as<T>(module: &Vec<ConfigKeyValue>, key: &str, default: T) -> T
	where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {

	get_key_value_as(module, key).unwrap_or(default)
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

pub fn get_config_file_mtime() -> Result<String, String> {
	let Some(config_dir_path) = get_config_path() else {
		Err("Could not determine the config directory. Make sure $HOME is set.".to_string())?
	};

	let config_path = config_dir_path + "/main.conf";

	let metadata = std::fs::metadata(config_path)
		.map_err(|e| format!("Error fetching config file metadata: {}", e))?;

	let modified = metadata.modified()
		.map_err(|e| format!("Error determining config file mtime: {}", e))?;

	let mtime = modified.duration_since(std::time::SystemTime::UNIX_EPOCH)
		.map_err(|e| format!("Config file mtime invalid: {}", e))?
		.as_millis().to_string();

	Ok(mtime)
}

pub fn load_config() -> Result<Vec<ConfigModule>, String> {
	let Some(config_dir_path) = get_config_path() else {
		Err("Could not determine the config directory. Make sure $HOME is set.".to_string())?
	};

	let config_path = format!("{}/main.conf", &config_dir_path);

	let config_contents = utils::read_string(&config_path).or_else(|_| {
		if let Err(e) = std::fs::create_dir_all(&config_dir_path) {
			Err(format!("Error creating path {}: {}", config_dir_path, e))?
		}

		let exampleconf = include_str!("example.conf");

		if let Err(e) = std::fs::write(&config_path, exampleconf) {
			Err(format!("Error creating config file {}: {}", config_path, e))?
		}

		Ok::<String, String>(exampleconf.to_string())
	})?;

	let config_lines = config_contents.lines();

	let mut found_general: bool = false;
	let mut output: Vec<ConfigModule> = Vec::new();

	for (linenum, line) in config_lines.enumerate() {
		if line.is_empty() { continue }

		// Ignore comment lines
		
		if line.starts_with('#') { continue }

		// Check for module name tag (eg. "[network]")

		if line.starts_with('[') && line.ends_with(']') {
			let new_module = line[1..line.len()-1].to_string();

			let mut module = ConfigModule {
				name: new_module,
				settings: Vec::new()
			};
			
			if &module.name == "general" {
				found_general = true;

				if let Ok(val) = get_config_file_mtime() {
					module.settings.push(ConfigKeyValue {
						key: "configmtime".to_string(),
						value: val
					})
				}
			}

			output.push(module);

			continue
		}

		// Parse key value pair

		let Some((key, value)) = line.split_once('=') else {
			Err(format!("Syntax error at line {}: expected key/value pair", linenum + 1))?
		};

		let mut value_trim = value.trim();

		if value_trim.is_empty() { continue }

		if value_trim.starts_with('"') && value_trim.ends_with('"') {
			value_trim = &value_trim[1..value_trim.len()-1];
		}

		if output.is_empty() {
			Err(format!("Syntax error at line {}: key/value pair found before any module tag", linenum + 1))?
		}

		output.last_mut().unwrap().settings.push(ConfigKeyValue {
			key: key.trim().to_string(),
			value: value_trim.to_string()
		});
	}

	if !found_general {
		Err("The config file is missing the [general] module.".to_string())?
	}

	Ok(output)
}

