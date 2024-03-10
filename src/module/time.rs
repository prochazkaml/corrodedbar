use crate::config;
use crate::modules;
use crate::getdata;

enum Data {
    FORMAT
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

	data.push(modules::ModuleData::TypeString(match config::getkeyvalue(config, "_format") {
		Some(val) => val.clone(),
		None => "%H:%M".to_string()
	}));

	Ok(data)
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(fmt, FORMAT, TypeString, data);

    Ok(Some(format!("{}", chrono::Local::now().format(&fmt))))
}

