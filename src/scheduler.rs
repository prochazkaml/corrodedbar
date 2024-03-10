use crate::config;
use crate::modules;
use crate::wm;
use std::time::{Duration, Instant};

pub fn run(config: &Vec<config::ConfigModule>, modules: &Vec<modules::ModuleRuntime>) {
    let mut counters: Vec<Duration> = Vec::new();
    let mut strings: Vec<Option<String>> = vec![None; modules.len()];
    
    for module in modules {
        counters.push(module.startdelay);
    }

    let start = Instant::now();

    let general = config::getmodule(&config, "general").unwrap();

    let defaults: Vec<String> = vec![" ".to_string(), " ".to_string(), "  ".to_string()];

    let leftpad = config::getkeyvaluedefault(&general, "leftpad", &defaults[0]);
    let rightpad = config::getkeyvaluedefault(&general, "rightpad", &defaults[1]);
    let delim = config::getkeyvaluedefault(&general, "delim", &defaults[2]);

    // TODO - signals

    // TODO - listen for config file changes and reload

    loop {
        // Run each scheduled module

        let mut elapsed = start.elapsed();

        for i in 0..modules.len() {
            if elapsed < counters[i] { continue; }

            //println!("Running module {}.", modules[i].module.name);

            strings[i] = match (modules[i].module.run)(&modules[i].data, counters[i]) {
                Ok(val) => val,
                Err(errmsg) => {
                    //println!(" -> {}", errmsg);
                    Some(errmsg)
                }
            };

            counters[i] += modules[i].interval;
        }

        // Generate the output string
        
        let mut output = leftpad.to_string();

        for i in 0..strings.len() {
            match &strings[i] {
                Some(val) => {
                    match &modules[i].icon {
                        Some(val) => { // TODO this is indentation hell
                            output += &val;
                            output += " ";
                        },
                        None => {}
                    }

                    output += val;

                    if i < strings.len() - 1 {
                        output += &delim;
                    }
                },
                None => {}
            }
        }

        output += &rightpad;

        //println!("'{}'", output);
        wm::setrootname(&output);

        // Figure out how much we have to sleep for
        
        let mut leastsleep = Duration::MAX;

        for i in 0..modules.len() {
            if counters[i] < leastsleep {
                leastsleep = counters[i];
            }
        }

        elapsed = start.elapsed();

        if leastsleep > elapsed {
            let sleep = leastsleep - elapsed;

            //println!("Going to sleep for {:?}.", sleep);

            std::thread::sleep(sleep);
        }
    }
}

