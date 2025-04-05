pub struct ConfigKeyValue {
	pub key: String,
	pub value: String
}

pub struct ConfigModule {
	pub name: String,
	pub settings: Vec<ConfigKeyValue>
}

pub fn getmodule<'a>(cfg: &'a Vec<ConfigModule>, name: &str) -> Option<&'a Vec<ConfigKeyValue>> {
	let namestr = name.to_string();

	for module in cfg {
		if module.name == namestr { return Some(&module.settings); }
	}

	None
}

pub fn getkeyvalue<'a>(module: &'a Vec<ConfigKeyValue>, key: &str) -> Option<String> {
	let keystr = key.to_string();

	for keyvalue in module {
		if keyvalue.key == keystr { return Some((keyvalue.value).clone()); }
	}
	
	None
}

pub fn getkeyvaluedefault<'a>(module: &'a Vec<ConfigKeyValue>, key: &str, default: &str) -> String {
	match getkeyvalue(module, key) {
		Some(val) => val,
		None => default.to_string()
	}
}

pub fn getkeyvalueas<T>(module: &Vec<ConfigKeyValue>, key: &str) -> Option<T>
	where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {

	match getkeyvalue(module, key) {
		Some(val) => Some(val.parse::<T>().expect("")),
		None => None
	}
}

pub fn getkeyvaluedefaultas<T>(module: &Vec<ConfigKeyValue>, key: &str, default: T) -> T
	where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {

	match getkeyvalueas(module, key) {
		Some(val) => val,
		None => default
	}
}

fn getxdgconfigpath() -> Option<String> {
	Some(std::env::var_os("XDG_CONFIG_HOME")?.into_string().ok()?)
}

fn getfakexdgconfigpath() -> Option<String> {
	match Some(std::env::var_os("HOME")?.into_string().ok()?) {
		Some(path) => Some(path + "/.config"),
		None => None
	}
}

fn getgeneralconfigpath() -> Option<String> {
	// Try the official XDG config path, if that fails, fall back to $HOME/.config

	match getxdgconfigpath() {
		Some(path) => Some(path),
		None => getfakexdgconfigpath()	
	}
}

fn getconfigpath() -> Option<String> {
	match getgeneralconfigpath() {
		Some(path) => Some(path + "/corrodedbar"),
		None => None
	}
}

pub fn getconfigfilemtime() -> Result<String, String> {
	let configdirpath = match getconfigpath() {
		Some(path) => path,
		None => {
			return Err("Could not determine the config directory. Make sure $HOME is set.".to_string());
		}
	};

	let configpath = configdirpath + "/main.conf";

	let metadata = match std::fs::metadata(configpath) {
		Ok(val) => val,
		Err(_) => {
			return Err("Error fetching config file metadata.".to_string());
		}
	};

	let modified = match metadata.modified() {
		Ok(val) => val,
		Err(_) => {
			return Err("Error determining config file mtime.".to_string());
		}
	};

	let mtime = match modified.duration_since(std::time::SystemTime::UNIX_EPOCH) {
		Ok(val) => val.as_millis().to_string(),
		Err(_) => {
			return Err("Config file mtime invalid.".to_string());
		}
	};

	Ok(mtime)
}

pub fn loadconfig() -> Result<Vec<ConfigModule>, String> {
	let configdirpath = match getconfigpath() {
		Some(path) => path,
		None => {
			return Err("Could not determine the config directory. Make sure $HOME is set.".to_string());
		}
	};

	let configpath = configdirpath.clone() + "/main.conf";

	let configcontents = match std::fs::read_to_string(configpath.clone()) {
		Ok(value) => value,
		Err(_) => {
			match std::fs::create_dir_all(configdirpath.clone()) {
				Ok(_) => {},
				Err(_) => { return Err(format!("Error creating path: {}", configdirpath)); }
			}

			let exampleconf = include_str!("example.conf");

			match std::fs::write(configpath.clone(), exampleconf) {
				Ok(_) => exampleconf.to_string(),
				Err(_) => { return Err(format!("Error creating config file: {}", configpath)); }
			}
		}
	};

	let configlines = configcontents.lines();

	let mut foundgeneral: bool = false;
	let mut currmodule: String;
	let mut output: Vec<ConfigModule> = Vec::new();

	for (linenum, line) in configlines.enumerate() {
		if line.len() <= 0 { continue; }

		// Ignore comment lines
		
		if line.chars().nth(0).unwrap() == '#' { continue; }

		// Check for module name tag (eg. "[network]")

		if line.chars().nth(0).unwrap() == '[' && line.chars().last().unwrap() == ']' {
			let newmodule = line[1..line.len()-1].to_string();

			currmodule = newmodule;
			
			output.push(ConfigModule {
				name: currmodule.clone(),
				settings: Vec::new()
			});

			if currmodule == "general".to_string() {
				foundgeneral = true;

				match getconfigfilemtime() {
					Ok(val) => output.last_mut().unwrap().settings.push(ConfigKeyValue {
						key: "configmtime".to_string(),
						value: val
					}),
					_ => {}
				}
			}

			continue;
		}

		// Parse key value pair

		let (key, value) = match line.split_once('=') {
			Some(arr) => arr,
			None => { return Err(format!("Syntax error at line {}: expected key/value pair", linenum + 1)); }
		};

		let mut valuetrim = value.trim();

		if valuetrim.len() <= 0 { continue; }

		if valuetrim.chars().nth(0).unwrap() == '"' && valuetrim.chars().last().unwrap() == '"' {
			valuetrim = &valuetrim[1..valuetrim.len()-1];
		}

		if output.len() <= 0 {
			return Err(format!("Syntax error at line {}: key/value pair found before any module tag", linenum + 1));
		}

		output.last_mut().unwrap().settings.push(ConfigKeyValue {
			key: key.trim().to_string(),
			value: valuetrim.to_string()
		});
	}

	if !foundgeneral {
		return Err("The config file is missing the [general] module.".to_string());
	}

	Ok(output)
}

