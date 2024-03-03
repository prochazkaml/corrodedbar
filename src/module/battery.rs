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
    let icons = std::collections::HashMap::from([
        ("Charging".to_string(), "🔌"),
        ("Full".to_string(), "✔️"),
        ("Discharging".to_string(), "🔋")
    ]);

    if let modules::ModuleData::TypeString(dev) = &data[DEVICE] {
        let status = utils::readline(format!("/sys/class/power_supply/{}/status", dev))?;
        let capacity = utils::readline(format!("/sys/class/power_supply/{}/capacity", dev))?;


        let icon = match icons.get(&status) {
            Some(val) => val,
            None => "?"
        };

        return Ok(Some(format!("{} {}%", icon, capacity)));
    }

	Err("Error during init".to_string())
}

