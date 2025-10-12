use crate::utils;

pub struct ConfigKeyValue {
	pub key: String,
	pub value: String
}

pub struct ConfigModule {
	pub name: String,
	pub settings: Vec<ConfigKeyValue>
}

pub fn getmodule<'a>(cfg: &'a Vec<ConfigModule>, name: &str) -> Option<&'a Vec<ConfigKeyValue>> {
	for module in cfg {
		if module.name == name { return Some(&module.settings); }
	}

	None
}

pub fn getkeyvalue<'a>(module: &'a Vec<ConfigKeyValue>, key: &str) -> Option<&'a str> {
	for keyvalue in module {
		if keyvalue.key == key { return Some(&keyvalue.value); }
	}
	
	None
}

pub fn getkeyvaluedefault<'a>(module: &'a Vec<ConfigKeyValue>, key: &str, default: &'a str) -> &'a str {
	getkeyvalue(module, key).unwrap_or(default)
}

pub fn getkeyvalueas<T>(module: &Vec<ConfigKeyValue>, key: &str) -> Option<T>
	where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {

	Some(getkeyvalue(module, key)?.parse::<T>().unwrap())
}

pub fn getkeyvaluedefaultas<T>(module: &Vec<ConfigKeyValue>, key: &str, default: T) -> T
	where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {

	getkeyvalueas(module, key).unwrap_or(default)
}

fn getxdgconfigpath() -> Option<String> {
	std::env::var_os("XDG_CONFIG_HOME")?.into_string().ok()
}

fn getfakexdgconfigpath() -> Option<String> {
	Some(std::env::var_os("XDG_CONFIG_HOME")?.into_string().ok()? + "/.config")
}

fn getgeneralconfigpath() -> Option<String> {
	// Try the official XDG config path, if that fails, fall back to $HOME/.config

	getxdgconfigpath().or_else(getfakexdgconfigpath)
}

fn getconfigpath() -> Option<String> {
	Some(getgeneralconfigpath()? + "/corrodedbar")
}

pub fn getconfigfilemtime() -> Result<String, String> {
	let Some(configdirpath) = getconfigpath() else {
		Err("Could not determine the config directory. Make sure $HOME is set.".to_string())?
	};

	let configpath = configdirpath + "/main.conf";

	let metadata = std::fs::metadata(configpath)
		.map_err(|e| format!("Error fetching config file metadata: {}", e))?;

	let modified = metadata.modified()
		.map_err(|e| format!("Error determining config file mtime: {}", e))?;

	let mtime = modified.duration_since(std::time::SystemTime::UNIX_EPOCH)
		.map_err(|e| format!("Config file mtime invalid: {}", e))?
		.as_millis().to_string();

	Ok(mtime)
}

pub fn loadconfig() -> Result<Vec<ConfigModule>, String> {
	let Some(configdirpath) = getconfigpath() else {
		Err("Could not determine the config directory. Make sure $HOME is set.".to_string())?
	};

	let configpath = format!("{}/main.conf", &configdirpath);

	let configcontents = utils::readstring(&configpath).or_else(|_| {
		if let Err(e) = std::fs::create_dir_all(&configdirpath) {
			Err(format!("Error creating path {}: {}", configdirpath, e))?
		}

		let exampleconf = include_str!("example.conf");

		if let Err(e) = std::fs::write(&configpath, exampleconf) {
			Err(format!("Error creating config file {}: {}", configpath, e))?
		}

		Ok::<String, String>(exampleconf.to_string())
	})?;

	let configlines = configcontents.lines();

	let mut foundgeneral: bool = false;
	let mut output: Vec<ConfigModule> = Vec::new();

	for (linenum, line) in configlines.enumerate() {
		if line.is_empty() { continue }

		// Ignore comment lines
		
		if line.starts_with('#') { continue }

		// Check for module name tag (eg. "[network]")

		if line.starts_with('[') && line.ends_with(']') {
			let newmodule = line[1..line.len()-1].to_string();

			let mut module = ConfigModule {
				name: newmodule,
				settings: Vec::new()
			};
			
			if &module.name == "general" {
				foundgeneral = true;

				if let Ok(val) = getconfigfilemtime() {
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

		let mut valuetrim = value.trim();

		if valuetrim.is_empty() { continue }

		if valuetrim.starts_with('"') && valuetrim.ends_with('"') {
			valuetrim = &valuetrim[1..valuetrim.len()-1];
		}

		if output.is_empty() {
			Err(format!("Syntax error at line {}: key/value pair found before any module tag", linenum + 1))?
		}

		output.last_mut().unwrap().settings.push(ConfigKeyValue {
			key: key.trim().to_string(),
			value: valuetrim.to_string()
		});
	}

	if !foundgeneral {
		Err("The config file is missing the [general] module.".to_string())?
	}

	Ok(output)
}

