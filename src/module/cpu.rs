use crate::config;
use crate::modules;
use crate::utils;
use crate::fmtopt;
use crate::getdata;

enum Data {
    TEMPDEVICE,
    FORMAT
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

	data.push(modules::ModuleData::TypeString(match config::getkeyvalue(config, "_tempdevice") {
		Some(val) => val.clone(),
		None => {
            return Err("Error: _tempdevice missing in the config".to_string());
        }
	}));

	data.push(modules::ModuleData::TypeString(match config::getkeyvalue(config, "_format") {
		Some(val) => val.clone(),
		None => "%t°C %f MHz".to_string()
	}));

	Ok(data)
}

fn gettemp(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(dev, TEMPDEVICE, TypeString, data);

    let currtemp: f64 = utils::readlineas(format!("{}", dev))?;

    Ok(Some(format!("{:.1}", currtemp / 1000.0)))
}

fn getfreq(_data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let file = match std::fs::read_to_string("/proc/cpuinfo") {
        Ok(val) => val,
        Err(errmsg) => { return Err(format!("File read error: {}", errmsg)); }
    };

    let lines = file.lines();
    
    let mut highestfreq: f64 = 0.0;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        if split.len() != 4 { continue; }
        
        if split[0] == "cpu" && split[1] == "MHz" {
            let freq = match split[3].parse::<f64>() {
                Ok(val) => val,
                Err(_) => { continue; }
            };
            
            if freq > highestfreq {
                highestfreq = freq;
            }
        }
    }
    
    Ok(if highestfreq > 0.0 {
        Some(format!("{:.0}", highestfreq))
    } else {
        None
    })
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(fmt, FORMAT, TypeString, data);

    let opts: &[utils::FormatOption] = &[
        fmtopt!('t', gettemp),
        fmtopt!('f', getfreq),
        // fmtopt!('p', getprocess), // TODO
    ];

    utils::format(fmt, opts, &data, _ts)
}

