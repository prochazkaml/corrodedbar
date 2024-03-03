use crate::config;
use crate::modules;

// used mem = MemTotal - MemAvailable

pub fn init(_config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    // TODO - format option

	Ok(data)
}

pub fn run(_data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let file = match std::fs::read_to_string("/proc/meminfo") {
        Ok(val) => val,
        Err(errmsg) => { return Err(format!("File read error: {}", errmsg)); }
    };

    let lines = file.lines();

    let mut total: i64 = -1;
    let mut avail: i64 = -1;

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
            avail = match split[1].parse::<i64>() {
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

    if total <= 0 || avail <= 0 {
        return Err("Could not determine".to_string());
    }

    if swaptotal > 0 && swapfree >= 0 {
        Ok(Some(format!("{}%/{}%",
            (total - avail) * 100 / total,
            (swaptotal - swapfree) * 100 / swaptotal
        )))
    } else {
        Ok(Some(format!("{}%",
            (total - avail) * 100 / total,
        )))
    }
}


