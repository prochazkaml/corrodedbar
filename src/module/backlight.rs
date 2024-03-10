use crate::config;
use crate::modules;
use crate::utils;
use crate::getdata;
use crate::configmandatory;

enum Data {
    DEVICE,
    SHOWRAW
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    configmandatory!("_device", TypeString, data, config);

	data.push(modules::ModuleData::TypeBool(config::getkeyvaluedefaultas(config, "_showraw", false)));

	Ok(data)
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(dev, DEVICE, TypeString, data);
    getdata!(raw, SHOWRAW, TypeBool, data);

    let curr: u32 = utils::readlineas(format!("/sys/class/backlight/{}/brightness", dev))?;

    Ok(Some(if *raw {
        format!("{}", curr)
    } else {
        let max: u32 = utils::readlineas(format!("/sys/class/backlight/{}/max_brightness", dev))?;

        format!("{}%", curr * 100 / max)
    }))
}

