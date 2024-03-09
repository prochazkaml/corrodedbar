use crate::config;
use crate::modules;
use crate::utils;
use crate::fmtopt;

const DEVICE: usize = 0;
const FORMAT: usize = 1;

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

	data.push(modules::ModuleData::TypeString(match config::getkeyvalue(config, "_device") {
		Some(val) => val.clone(),
		None => {
            return Err("Error: _device missing in the config".to_string());
        }
	}));

	data.push(modules::ModuleData::TypeString(match config::getkeyvalue(config, "_format") {
		Some(val) => val.clone(),
		None => "%i %p%% (%w)".to_string()
	}));

	Ok(data)
}

fn geticon(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let modules::ModuleData::TypeString(dev) = &data[DEVICE] else {
        return modules::init_error_msg();
    };
    
    let icons = std::collections::HashMap::from([
        ("Charging".to_string(), "🔌"),
        ("Full".to_string(), "✔️"),
        ("Discharging".to_string(), "🔋")
    ]);

    let status = utils::readline(format!("/sys/class/power_supply/{}/status", dev))?;
    
    Ok(Some(match icons.get(&status) {
        Some(val) => val,
        None => "?"
    }.to_string()))
}

fn getpercentage(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let modules::ModuleData::TypeString(dev) = &data[DEVICE] else {
        return modules::init_error_msg();
    };
    
    Ok(Some(utils::readline(format!("/sys/class/power_supply/{}/capacity", dev))?))
}

fn getpower(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let modules::ModuleData::TypeString(dev) = &data[DEVICE] else {
        return modules::init_error_msg();
    };

    let file = utils::readline(format!("/sys/class/power_supply/{}/power_now", dev))?;

    let power = match file.parse::<f64>() {
        Ok(val) => val,
        Err(_) => { return Err("Format error".to_string()); }
    };
    
    Ok(Some(format!("{:.1} W", power / 1000000.0)))
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let modules::ModuleData::TypeString(fmt) = &data[FORMAT] else {
        return modules::init_error_msg();
    };

    let opts: &[utils::FormatOption] = &[
        fmtopt!('i', geticon),
        fmtopt!('p', getpercentage),
        fmtopt!('w', getpower),
    ];

    utils::format(fmt, opts, data, _ts)
}

