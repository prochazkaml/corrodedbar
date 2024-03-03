use crate::config;
use crate::modules;

use pulsectl::controllers::SourceController;
use pulsectl::controllers::AppControl;

pub fn init(_config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

	Ok(data)
}

pub fn run(_data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let mut handler = match SourceController::create() {
        Ok(val) => val,
        Err(_) => { return Err("PulseAudio conn error".to_string()); }
    };

    let apps = match handler.list_applications() {
        Ok(val) => val,
        Err(_) => { return Ok(None); }
    };

    if apps.len() <= 0 {
        return Ok(None);
    } else {
        return Ok(Some("active".to_string()));
    }
}

