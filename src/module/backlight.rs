use crate::config;
use crate::modules;
use crate::utils;

const DEVICE: usize = 0;

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

	data.push(modules::ModuleData::TypeString(match config::getkeyvalue(config, "_device") {
		Some(val) => val.clone(),
		None => {
            return Err("Error: _device missing in the config".to_string());
        }
	}));

	Ok(data)
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    // TODO - add option for displaying as a percentage

    if let modules::ModuleData::TypeString(dev) = &data[DEVICE] {
        let curr = utils::readline(format!("/sys/class/backlight/{}/brightness", dev))?;

        return Ok(Some(format!("{}", curr)));
    }

	Err("Error during init".to_string())
}

