use crate::formatter;
use crate::fmtopt;

pub fn readstring(path: &str) -> Result<String, String> {
    // Yes, this is just a fancy wrapper function.
    
    match std::fs::read_to_string(path) {
        Ok(val) => Ok(val),
        Err(errmsg) => Err(format!("File read error: {}", errmsg))
    }
}

pub fn readline(path: &str) -> Result<String, String> {
    let file = readstring(path)?;

    match file.lines().next() {
        Some(val) => Ok(val.to_string()),
        None => { return Err("File parse error".to_string()); }
    }
}

pub fn readlineas<T>(path: &str) -> Result<T, String>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    
    match readline(path)?.split(' ').next().unwrap().parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => { return Err("Format error".to_string()); }
    }
}

pub fn formatduration(fmt: &String, dur: f64) -> Result<Option<String>, String> {
	let timeint = Ok(Some((dur * 1000.0) as i64));

    formatter::format(&fmt, |tag| {
		match tag {
			'd' => fmtopt!(i64 timeint.clone(), "[d86400000]"),
			'H' => fmtopt!(i64 timeint.clone(), "[d3600000 r24 z2]"),
			'h' => fmtopt!(i64 timeint.clone(), "[d3600000]"),
			'M' => fmtopt!(i64 timeint.clone(), "[d60000 r60 z2]"),
			'm' => fmtopt!(i64 timeint.clone(), "[d60000]"),
			'S' => fmtopt!(i64 timeint.clone(), "[d1000 r60 z2]"),
			's' => fmtopt!(i64 timeint.clone(), "[d1000]"),
			'L' => fmtopt!(i64 timeint.clone(), "[d1 r60 z2]"),
			'l' => fmtopt!(i64 timeint.clone(), "[d1]"),
			_ => Ok(None)
		}
	})
}

