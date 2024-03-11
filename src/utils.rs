use crate::modules;
use crate::utils;

pub struct FormatOption {
    pub id: char,
    pub generate: fn(&Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String>
}

#[macro_export]
macro_rules! fmtopt {
	($char:literal, $fnname:ident) => {
		utils::FormatOption {
			id: $char,
            generate: $fnname
		}
	}
}

fn callformatfn(c: char, fmtopts: &[FormatOption], data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<String, String> {
    for opt in fmtopts {
        if opt.id == c {
            match (opt.generate)(data, _ts)? {
                Some(val) => return Ok(val),
                None => return Ok("".to_string())
            }
        }
    }

    Ok(format!("%{}", c))
}

pub fn format(fmt: &String, fmtopts: &[FormatOption], data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let mut out = String::new();
    
    // TODO - custom number formats?

    let mut controlchar = false;

    for c in fmt.chars() { match controlchar {
        false => { 
            if c == '%' {
                controlchar = true;
            } else {
                out.push(c);
            }
        },
        true => {
            match c {
                '%' => out.push('%'),
                _ => out += &callformatfn(c, fmtopts, data, _ts)?
            };

            controlchar = false;
        }
    }}

    return Ok(Some(out));
}

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
    
    // TODO - custom delimiters

    match readline(path)?.split(' ').next().unwrap().parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => { return Err("Format error".to_string()); }
    }
}

macro_rules! genuptimefun {
	($fnname:ident, $cap: literal, $div:literal, $mod:literal, $decimals:literal) => {
        pub fn $fnname(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
            let modules::ModuleData::TypeFloat64(time) = &data[0] else {
                return modules::init_error_msg();
            };

            let timeint = (time * 1000.0) as u64;

            Ok(Some(if $cap {
                format!(concat!("{:0>", $decimals, "}"), timeint / $div % $mod)
            } else {
                format!("{}", timeint / $div)
            }))
        }
	}
}

genuptimefun!(getday, false, 86400000, 0, 0);
genuptimefun!(gethour, false, 3600000, 0, 0);
genuptimefun!(gethourcapped, true, 3600000, 60, 2);
genuptimefun!(getminute, false, 60000, 0, 0);
genuptimefun!(getminutecapped, true, 60000, 60, 2);
genuptimefun!(getsecond, false, 1000, 0, 0);
genuptimefun!(getsecondcapped, true, 1000, 60, 2);
genuptimefun!(getmillis, false, 1, 0, 0);
genuptimefun!(getmilliscapped, true, 1, 1000, 3);

pub fn formatduration(fmt: &String, dur: f64) -> Result<Option<String>, String> {
    let mut data: Vec<modules::ModuleData> = Vec::new();
    data.push(modules::ModuleData::TypeFloat64(dur));

    let opts: &[utils::FormatOption] = &[
        fmtopt!('d', getday),
        fmtopt!('H', gethourcapped),
        fmtopt!('h', gethour),
        fmtopt!('M', getminutecapped),
        fmtopt!('m', getminute),
        fmtopt!('S', getsecondcapped),
        fmtopt!('s', getsecond),
        fmtopt!('L', getmilliscapped),
        fmtopt!('l', getmillis)
    ];

    format(&fmt, opts, &data, std::time::Duration::MAX)
}

