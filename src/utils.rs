use crate::modules;
use crate::utils;

type FmtGenStringFn = fn(&Vec<modules::ModuleData>, std::time::Duration) -> Result<Option<String>, String>;
type FmtGenString = FmtGenStringFn;

type FmtGenFloat64Fn = fn(&Vec<modules::ModuleData>, std::time::Duration, Option<&String>) -> Result<Option<f64>, String>;
pub struct FmtGenFloat64 {
    pub fun: FmtGenFloat64Fn,
    pub defaultfmt: Option<String>
}

type FmtGenInt64Fn = fn(&Vec<modules::ModuleData>, std::time::Duration, Option<&String>) -> Result<Option<f64>, String>;
pub struct FmtGenInt64 {
    pub fun: FmtGenInt64Fn,
    pub defaultfmt: Option<String>
}

pub enum FormatGenerator {
    OutputString(FmtGenString),
    OutputFloat64(FmtGenFloat64),
    OutputInt64(FmtGenInt64)
}

pub struct FormatOption {
    pub id: char,
    pub generate: FormatGenerator
}

#[macro_export]
macro_rules! fmtopt {
	($char:literal, String $fnname:ident) => {
		utils::FormatOption {
			id: $char,
            generate: utils::FormatGenerator::OutputString($fnname)
		}
	};
	($char:literal, f64 $fnname:ident) => {
		utils::FormatOption {
			id: $char,
            generate: utils::FormatGenerator::OutputFloat64(utils::FmtGenFloat64 {
                fun: $fnname,
                defaultfmt: None
            })
		}
	};
	($char:literal, f64 $fnname:ident, $defaultfmt:literal) => {
		utils::FormatOption {
			id: $char,
            generate: utils::FormatGenerator::OutputFloat64(utils::FmtGenFloat64 {
                fun: $fnname,
                defaulttype: None,
                defaultfmt: Some($defaultfmt.to_string())
            })
		}
	};
}

fn callformatfnstr(fun: &FmtGenString, data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<String, String> {
    /*
     * String format syntax: `%T`
     *   T = token
     */

    match fun(data, ts)? {
        Some(val) => return Ok(val),
        None => return Ok("".to_string())
    }
}

fn callformatfnf64(fun: &FmtGenFloat64, data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<String, String> {
    /*
     * Float format syntax: `%T[dD pP zZ]`
     *   T = token
     *   D = divisor, 1 to disable (duh)
     *       1 by default
     *   P = number of output decimal places
     *       0 by default
     *   Z = minimum number of digits before the decimal point (zero-pad)
     *       0 by default
     *
     *   result = fnoutput / D; rounded to P decimal places, zero-padded to Z digits
     */

    let result: f64 = match (fun.fun)(data, ts, None)? {
        Some(val) => val,
        None => return Ok("".to_string())
    };

    // TODO

    //println!("{} {}", result, fun.defaultfmt);

    Ok("".to_string())
}

fn callformatfni64(fun: &FmtGenInt64, data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<String, String> {
    /*
     * Float format syntax: `%T[dD rR zZ]`
     *   T = token
     *   D = divisor, 1 to disable (duh)
     *       1 by default
     *   R = divisor for remainder calculation, 0 to disable
     *       0 by default
     *   Z = minimum number of digits before the decimal point (zero-pad)
     *       0 by default
     *
     *   result = (fnoutput / D) % R; zero-padded to Z digits
     */

    let result: f64 = match (fun.fun)(data, ts, None)? {
        Some(val) => val,
        None => return Ok("".to_string())
    };

    // TODO

    //println!("{} {}", result, fun.defaultfmt);

    Ok("".to_string())
}

fn callformatfn(c: char, fmtopts: &[FormatOption], data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<String, String> {
    for opt in fmtopts {
        if opt.id == c {
            match &opt.generate {
                FormatGenerator::OutputString(fun) => return callformatfnstr(&fun, data, ts),
                FormatGenerator::OutputFloat64(fun) => return callformatfnf64(&fun, data, ts),
                FormatGenerator::OutputInt64(fun) => return callformatfni64(&fun, data, ts)
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

fn testfloat64(_data: &Vec<modules::ModuleData>, _ts: std::time::Duration, _type: Option<&String>) -> Result<Option<f64>, String> {
    Ok(Some(0.0 as f64))
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
        fmtopt!('a', f64 testfloat64),
        fmtopt!('d', String getday),
        fmtopt!('H', String gethourcapped),
        fmtopt!('h', String gethour),
        fmtopt!('M', String getminutecapped),
        fmtopt!('m', String getminute),
        fmtopt!('S', String getsecondcapped),
        fmtopt!('s', String getsecond),
        fmtopt!('L', String getmilliscapped),
        fmtopt!('l', String getmillis)
    ];

    format(&fmt, opts, &data, std::time::Duration::MAX)
}

