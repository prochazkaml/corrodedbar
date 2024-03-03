use crate::config;
use crate::modules;
use crate::utils;

pub fn init(_config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let data: Vec<modules::ModuleData> = Vec::new();

    // TODO - format option

	Ok(data)
}

pub fn run(_data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let file = utils::readline("/proc/uptime".to_string())?;

    let uptime = match file.split(' ').next().unwrap().parse::<f64>() {
        Ok(val) => val as u64,
        Err(_) => { return Err("Format error".to_string()); }
    };

    return Ok(Some(format!("{}d {}h {}m",
        uptime / 86400,
        (uptime / 3600) % 24,
        (uptime / 60) % 60
    )));
}

