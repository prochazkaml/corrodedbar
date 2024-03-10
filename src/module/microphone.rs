use crate::config;
use crate::modules;
use crate::getdata;
use crate::configoptional;

use pulsectl::controllers::SourceController;
use pulsectl::controllers::AppControl;

enum Data {
    ACTIVESTRING
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    configoptional!("_active", TypeString, "active", data, config);

	Ok(data)
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let mut handler = match SourceController::create() {
        Ok(val) => val,
        Err(_) => { return Err("PulseAudio conn error".to_string()); }
    };

    let apps = match handler.list_applications() {
        Ok(val) => val,
        Err(_) => { return Ok(None); }
    };

    getdata!(activestr, ACTIVESTRING, TypeString, data);

    if apps.len() <= 0 {
        return Ok(None);
    } else {
        return Ok(Some(activestr.to_string()));
    }
}

