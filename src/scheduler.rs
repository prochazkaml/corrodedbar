use crate::config;
use crate::modules;
use std::time::{Duration, Instant};

pub fn run(config: &Vec<config::ConfigModule>, modules: &Vec<modules::ModuleRuntime>) {
    let mut counters: Vec<Duration> = Vec::new();
    let mut strings: Vec<String> = vec!["".to_string(); modules.len()];
    
    for module in modules {
        counters.push(module.startdelay);
    }

    let start = Instant::now();

    loop {
        // Run each scheduled module

        let elapsed = start.elapsed();

        for i in 0..modules.len() {
            if elapsed >= counters[i] {
                println!("Running module {}.", modules[i].module.name);
                println!("{}", (modules[i].module.run)(&modules[i].data, counters[i]).ok().unwrap().unwrap());
                counters[i] += modules[i].interval;
            }
        }

        // TODO - Generate the output string
        
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

