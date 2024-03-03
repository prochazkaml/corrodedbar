use crate::config;
use crate::modules;

use pulsectl::controllers::SinkController;
use pulsectl::controllers::DeviceControl;

pub fn init(_config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let data: Vec<modules::ModuleData> = Vec::new();

	Ok(data)
}

pub fn run(_data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let mut handler = match SinkController::create() {
        Ok(val) => val,
        Err(_) => { return Err("PulseAudio conn error".to_string()); }
    };

    let dev = match handler.get_default_device() {
        Ok(val) => val,
        Err(_) => { return Err("Error getting default device".to_string()); }
    };

    Ok(Some(format!("{}", if dev.mute {
        "off".to_string()
    } else {
        dev.volume.get()[0].to_string().trim().to_string()
    })))
}

