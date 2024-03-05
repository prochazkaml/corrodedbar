use crate::config;
use crate::modules;
use crate::utils;

const TEMPDEVICE: usize = 0;

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    // TODO - make the temperature, clock frequency & heaviest process readout not mandatory

	data.push(modules::ModuleData::TypeString(match config::getkeyvalue(config, "_tempdevice") {
		Some(val) => val.clone(),
		None => {
            return Err("Error: _tempdevice missing in the config".to_string());
        }
	}));

	Ok(data)
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let mut output: Vec<String> = Vec::new();

    // CPU temperature

    if let modules::ModuleData::TypeString(dev) = &data[TEMPDEVICE] {
        let currtempstr = utils::readline(format!("{}", dev))?;

        let currtemp = match currtempstr.parse::<i32>() {
            Ok(val) => val,
            Err(_) => { return Err("Format error".to_string()); }
        };

        output.push(format!("{:.1}°C", (currtemp as f64) / 1000.0));
    }

    // Peak CPU clock frequency

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
    
    if highestfreq > 0.0 {
        output.push(format!("{:.0} MHz", highestfreq));
    }

    // Heaviest process
    
    // TODO

    // Assemble output

    let mut outputstr = String::new();

    if output.len() == 0 {
        return Ok(None);
    }

    for i in 0..output.len() {
        outputstr += &output[i];
        if i < output.len() - 1 {
            outputstr += " ";
        }
    }

    Ok(Some(outputstr))
}

