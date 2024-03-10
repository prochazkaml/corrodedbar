use crate::config;
use crate::modules;
use crate::utils;
use crate::fmtopt;
use crate::getdata;
use crate::configmandatory;
use crate::configoptional;

enum Data {
    DEVICE,
    FORMAT
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();
    
    configmandatory!("_device", TypeString, data, config);
    configoptional!("_format", TypeString, "%i %p%% (%w W %e)", data, config);

	Ok(data)
}

fn geticon(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(dev, DEVICE, TypeString, data);
    
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
    getdata!(dev, DEVICE, TypeString, data);
    
    Ok(Some(utils::readline(format!("/sys/class/power_supply/{}/capacity", dev))?))
}

fn getpower(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(dev, DEVICE, TypeString, data);

    let power: f64 = utils::readlineas(format!("/sys/class/power_supply/{}/power_now", dev))?;

    Ok(Some(format!("{:.1}", power / 1000000.0)))
}

fn getestimate(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let empty = Ok(Some("--:--".to_string()));

    getdata!(dev, DEVICE, TypeString, data);

    let status = utils::readline(format!("/sys/class/power_supply/{}/status", dev))?;

    let power: u64 = utils::readlineas(format!("/sys/class/power_supply/{}/power_now", dev))?;
            
    if power == 0 {
        return empty;
    }

    let energynow: u64 = utils::readlineas(format!("/sys/class/power_supply/{}/energy_now", dev))?;
    
    match status.as_str() {
        "Charging" => {
            let energyfull: u64 = utils::readlineas(format!("/sys/class/power_supply/{}/energy_full", dev))?;

            let tillfull = (energyfull - energynow) * 3600 / power;

            Ok(Some(format!("{:0>2}:{:0>2}",
                tillfull / 3600,
                (tillfull / 60) % 60
            )))
        },
        "Discharging" => {
            let tillempty = energynow * 3600 / power;

            Ok(Some(format!("{:0>2}:{:0>2}",
                tillempty / 3600,
                (tillempty / 60) % 60
            )))
        },
        _ => { empty }
    }
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(fmt, FORMAT, TypeString, data);

    let opts: &[utils::FormatOption] = &[
        fmtopt!('i', geticon),
        fmtopt!('p', getpercentage),
        fmtopt!('w', getpower),
        fmtopt!('e', getestimate)
    ];

    utils::format(fmt, opts, data, _ts)
}

