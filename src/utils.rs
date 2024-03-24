use crate::modules;
use crate::formatter;
use crate::fmtopt;

pub fn readstring(path: String) -> Result<String, String> {
    // Yes, this is just a fancy wrapper function.
    
    match std::fs::read_to_string(path) {
        Ok(val) => Ok(val),
        Err(errmsg) => Err(format!("File read error: {}", errmsg))
    }
}

pub fn readline(path: String) -> Result<String, String> {
    let file = readstring(path)?;

    match file.lines().next() {
        Some(val) => Ok(val.to_string()),
        None => { return Err("File parse error".to_string()); }
    }
}

pub fn readlineas<T>(path: String) -> Result<T, String>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    
    match readline(path)?.split(' ').next().unwrap().parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => { return Err("Format error".to_string()); }
    }
}

fn parsetime(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<i64>, String> {
    let modules::ModuleData::TypeFloat64(time) = &data[0] else {
        return Err(modules::internalerrormsg());
    };

    let timeint = (time * 1000.0) as i64;

    Ok(Some(timeint))
}

pub fn formatduration(fmt: &String, dur: f64) -> Result<Option<String>, String> {
    let mut data: Vec<modules::ModuleData> = Vec::new();
    data.push(modules::ModuleData::TypeFloat64(dur));

    let opts: &[formatter::FormatOption] = &[
        fmtopt!('d', i64 parsetime, "[d86400000]"),
        fmtopt!('H', i64 parsetime, "[d3600000 r24 z2]"),
        fmtopt!('h', i64 parsetime, "[d3600000]"),
        fmtopt!('M', i64 parsetime, "[d60000 r60 z2]"),
        fmtopt!('m', i64 parsetime, "[d60000]"),
        fmtopt!('S', i64 parsetime, "[d1000 r60 z2]"),
        fmtopt!('s', i64 parsetime, "[d1000]"),
        fmtopt!('L', i64 parsetime, "[d1 r60 z2]"),
        fmtopt!('l', i64 parsetime, "[d1]"),
    ];

    formatter::format(&fmt, opts, &data, std::time::Duration::MAX)
}

