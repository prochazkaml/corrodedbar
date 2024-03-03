use crate::config;
use crate::modules;
use std::time::{Duration, Instant};

pub fn run(config: &Vec<config::ConfigModule>, modules: &Vec<modules::ModuleRuntime>) {
    let mut counters: Vec<Duration> = Vec::new();
    let mut strings: Vec<Option<String>> = vec![None; modules.len()];
    
    for module in modules {
        counters.push(module.startdelay);
    }

    let start = Instant::now();

    let general = config::getmodule(&config, "general").unwrap();

    // TODO - config::getkeyvaluedefault, config::getkeyvaluedefaultas

    let leftpad = match config::getkeyvalue(&general, "leftpad") {
        Some(val) => val,
        None => " "
    };

    let rightpad = match config::getkeyvalue(&general, "rightpad") {
        Some(val) => val,
        None => " "
    };

    let delim = match config::getkeyvalue(&general, "delim") {
        Some(val) => val,
        None => "  "
    };

    loop {
        // Run each scheduled module

        let elapsed = start.elapsed();

        for i in 0..modules.len() {
            if elapsed < counters[i] { continue; }

            println!("Running module {}.", modules[i].module.name);

            strings[i] = match (modules[i].module.run)(&modules[i].data, counters[i]) {
                Ok(val) => val,
                Err(errmsg) => {
                    println!(" -> {}", errmsg);
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
                    output += val;
                    if i < strings.len() - 1 {
                        output += delim;
                    }
                },
                None => {}
            }
        }

        output += rightpad;

        println!("'{}'", output);

        // Figure out how much we have to sleep for
        
        let mut leastsleep = Duration::MAX;

        for i in 0..modules.len() {
            if counters[i] < leastsleep {
                leastsleep = counters[i];
            }
        }

        let sleep = leastsleep - start.elapsed();

        println!("Going to sleep for {:?}.", sleep);

        std::thread::sleep(sleep);
    }
}

