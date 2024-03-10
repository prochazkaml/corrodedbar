use crate::config;
use crate::modules;
use crate::utils;
use crate::fmtopt;

const FORMAT: usize = 0;

const SUB_PHYSICALTOTAL: usize = 1;
const SUB_PHYSICALFREE: usize = 2;
const SUB_SWAPTOTAL: usize = 3;
const SUB_SWAPFREE: usize = 4;

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

	data.push(modules::ModuleData::TypeString(match config::getkeyvalue(config, "_format") {
		Some(val) => val.clone(),
		None => "%p%%/%s%%".to_string()
	}));

	Ok(data)
}

macro_rules! genmemoryfun {
	($fnname:ident, $calculateused: literal, $freefield:expr, $totalfield:expr) => {
        pub fn $fnname(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
            let modules::ModuleData::TypeInt64(total) = &data[$totalfield] else {
                return modules::init_error_msg();
            };

            let modules::ModuleData::TypeInt64(free) = &data[$freefield] else {
                return modules::init_error_msg();
            };

            Ok(Some(if *free >= 0 && *total > 0 {
                if $calculateused {
                    format!("{}", (total - free) * 100 / total)
                } else {
                    format!("{}", free * 100 / total)
                }
            } else {
                "??".to_string()
            }))
        }
	}
}

genmemoryfun!(getusedphysical, true, SUB_PHYSICALFREE, SUB_PHYSICALTOTAL);
genmemoryfun!(getfreephysical, false, SUB_PHYSICALFREE, SUB_PHYSICALTOTAL);
genmemoryfun!(getusedswap, true, SUB_SWAPFREE, SUB_SWAPTOTAL);
genmemoryfun!(getfreeswap, false, SUB_SWAPFREE, SUB_SWAPTOTAL);

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let modules::ModuleData::TypeString(fmt) = &data[FORMAT] else {
        return modules::init_error_msg();
    };

    let file = match std::fs::read_to_string("/proc/meminfo") {
        Ok(val) => val,
        Err(errmsg) => { return Err(format!("File read error: {}", errmsg)); }
    };

    let lines = file.lines();

    let mut total: i64 = -1;
    let mut free: i64 = -1;

    let mut swaptotal: i64 = -1;
    let mut swapfree: i64 = -1;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        if split.len() != 3 { continue; }

        if split[0] == "MemTotal:" {
            total = match split[1].parse::<i64>() {
                Ok(val) => val,
                Err(_) => { return Err("Format error".to_string()); }
            }
        }

        if split[0] == "MemAvailable:" {
            free = match split[1].parse::<i64>() {
                Ok(val) => val,
                Err(_) => { return Err("Format error".to_string()); }
            }
        }

        if split[0] == "SwapTotal:" {
            swaptotal = match split[1].parse::<i64>() {
                Ok(val) => val,
                Err(_) => { return Err("Format error".to_string()); }
            }
        }

        if split[0] == "SwapFree:" {
            swapfree = match split[1].parse::<i64>() {
                Ok(val) => val,
                Err(_) => { return Err("Format error".to_string()); }
            }
        }
    }

    let mut subdata = data.clone();
    subdata.push(modules::ModuleData::TypeInt64(total));
    subdata.push(modules::ModuleData::TypeInt64(free));
    subdata.push(modules::ModuleData::TypeInt64(swaptotal));
    subdata.push(modules::ModuleData::TypeInt64(swapfree));

    // TODO - display as percentage **or** specific units

    let opts: &[utils::FormatOption] = &[
        fmtopt!('p', getusedphysical),
        fmtopt!('P', getfreephysical),
        fmtopt!('s', getusedswap),
        fmtopt!('S', getfreeswap),
    ];

    utils::format(fmt, opts, &subdata, _ts)
}


