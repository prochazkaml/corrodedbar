use crate::config;
use crate::modules;
use crate::utils;
use crate::getdata;
use crate::configoptional;

enum Data {
    FORMAT
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    configoptional!("_format", TypeString, "%dd %Hh %Mm", data, config);

	Ok(data)
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(fmt, FORMAT, TypeString, data);

    let uptime: f64 = utils::readlineas("/proc/uptime".to_string())?;

    utils::formatduration(fmt, uptime)
}

