use crate::config;
use crate::modules;
use crate::utils;
use crate::fmtopt;
use crate::getdata;
use crate::configoptional;

enum Data {
    FORMAT,
    SUBPHYSICALTOTAL,
    SUBPHYSICALFREE,
    SUBSWAPTOTAL,
    SUBSWAPFREE
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    configoptional!("_format", TypeString, "%p%%/%s%%", data, config);

	Ok(data)
}

fn getmeminfo(total: f64, free: f64, percentage: bool, calculateused: bool) -> Result<Option<f64>, String> {
    if free >= 0.0 && total > 0.0 {
        if calculateused {
            if percentage {
                Ok(Some((total - free) * 100.0 / total))
            } else {
                Ok(Some(total - free))
            }
        } else {
            if percentage {
                Ok(Some(free * 100.0 / total))
            } else {
                Ok(Some(free))
            }
        }
    } else {
        Ok(None)
    }
}

macro_rules! genmemoryfun {
	($fnname:ident, $percentage:literal, $calculateused:literal, $freefield:ident, $totalfield:ident) => {
        pub fn $fnname(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<f64>, String> {
            getdata!(total, $totalfield, TypeFloat64, data);
            getdata!(free, $freefield, TypeFloat64, data);

            getmeminfo(*total, *free, $percentage, $calculateused)
        }
	}
}

genmemoryfun!(getusedphysical, false, true, SUBPHYSICALFREE, SUBPHYSICALTOTAL);
genmemoryfun!(getfreephysical, false, false, SUBPHYSICALFREE, SUBPHYSICALTOTAL);
genmemoryfun!(getusedswap, false, true, SUBSWAPFREE, SUBSWAPTOTAL);
genmemoryfun!(getfreeswap, false, false, SUBSWAPFREE, SUBSWAPTOTAL);
genmemoryfun!(getusedphysicalperc, true, true, SUBPHYSICALFREE, SUBPHYSICALTOTAL);
genmemoryfun!(getfreephysicalperc, true, false, SUBPHYSICALFREE, SUBPHYSICALTOTAL);
genmemoryfun!(getusedswapperc, true, true, SUBSWAPFREE, SUBSWAPTOTAL);
genmemoryfun!(getfreeswapperc, true, false, SUBSWAPFREE, SUBSWAPTOTAL);

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(fmt, FORMAT, TypeString, data);

    let file = match std::fs::read_to_string("/proc/meminfo") {
        Ok(val) => val,
        Err(errmsg) => { return Err(format!("File read error: {}", errmsg)); }
    };

    let lines = file.lines();

    let mut total: f64 = -1.0;
    let mut free: f64 = -1.0;

    let mut swaptotal: f64 = -1.0;
    let mut swapfree: f64 = -1.0;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        if split.len() != 3 { continue; }

        if split[0] == "MemTotal:" {
            total = match split[1].parse::<f64>() {
                Ok(val) => val,
                Err(_) => { return Err("Format error".to_string()); }
            }
        }

        if split[0] == "MemAvailable:" {
            free = match split[1].parse::<f64>() {
                Ok(val) => val,
                Err(_) => { return Err("Format error".to_string()); }
            }
        }

        if split[0] == "SwapTotal:" {
            swaptotal = match split[1].parse::<f64>() {
                Ok(val) => val,
                Err(_) => { return Err("Format error".to_string()); }
            }
        }

        if split[0] == "SwapFree:" {
            swapfree = match split[1].parse::<f64>() {
                Ok(val) => val,
                Err(_) => { return Err("Format error".to_string()); }
            }
        }
    }

    let mut subdata = data.clone();
    subdata.push(modules::ModuleData::TypeFloat64(total));
    subdata.push(modules::ModuleData::TypeFloat64(free));
    subdata.push(modules::ModuleData::TypeFloat64(swaptotal));
    subdata.push(modules::ModuleData::TypeFloat64(swapfree));

    let opts: &[utils::FormatOption] = &[
        fmtopt!('p', f64 getusedphysicalperc),
        fmtopt!('P', f64 getfreephysicalperc),
        fmtopt!('s', f64 getusedswapperc),
        fmtopt!('S', f64 getfreeswapperc),
        fmtopt!('h', f64 getusedphysical),
        fmtopt!('H', f64 getfreephysical),
        fmtopt!('w', f64 getusedswap),
        fmtopt!('W', f64 getfreeswap),
    ];

    utils::format(fmt, opts, &subdata, _ts)
}

