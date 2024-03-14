use crate::config;
use crate::modules;
use crate::utils;
use crate::fmtopt;
use crate::getdata;
use crate::configmandatory;
use crate::configoptional;

enum Data {
    TEMPDEVICE,
    FORMAT,
    SUBPROCCPUINFO
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    configmandatory!("_tempdevice", TypeString, data, config);
    configoptional!("_format", TypeString, "%t°C %F MHz", data, config);

	Ok(data)
}

fn gettemp(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<f64>, String> {
    getdata!(dev, TEMPDEVICE, TypeString, data);

    let currtemp: f64 = utils::readlineas(format!("{}", dev))?;

    Ok(Some(currtemp))
}

fn getfreq(data: &Vec<modules::ModuleData>, highest: bool) -> Result<Option<f64>, String> {
    getdata!(file, SUBPROCCPUINFO, TypeString, data);

    let lines = file.lines();

    let default: f64 = if highest { 0.0 } else { 1000000.0 };

    let mut target: f64 = default;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        if split.len() != 4 { continue; }
        
        if split[0] == "cpu" && split[1] == "MHz" {
            let freq = match split[3].parse::<f64>() {
                Ok(val) => val,
                Err(_) => { continue; }
            };
            
            if (freq > target && highest) || (freq < target && !highest) {
                target = freq;
            }
        }
    }
    
    Ok(if target != default {
        Some(target)
    } else {
        None
    })
}

fn gethighestfreq(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<f64>, String> {
    getfreq(data, true)
}

fn getlowestfreq(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<f64>, String> {
    getfreq(data, false)
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(fmt, FORMAT, TypeString, data);
    
    let mut subdata = data.clone();
    subdata.push(modules::ModuleData::TypeString(utils::readstring("/proc/cpuinfo".to_string())?));

    let opts: &[utils::FormatOption] = &[
        fmtopt!('t', f64 gettemp, "[d1000 p1]"),
        fmtopt!('F', f64 gethighestfreq),
        fmtopt!('f', f64 getlowestfreq),
        // fmtopt!('p', String getprocess), // TODO
    ];

    utils::format(fmt, opts, &subdata, _ts)
}

