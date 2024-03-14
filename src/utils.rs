use crate::modules;
use crate::utils;

type CharIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

type FmtGenStringFn = fn(&Vec<modules::ModuleData>, std::time::Duration) -> Result<Option<String>, String>;
type FmtGenString = FmtGenStringFn;

type FmtGenFloat64Fn = fn(&Vec<modules::ModuleData>, std::time::Duration) -> Result<Option<f64>, String>;
pub struct FmtGenFloat64 {
    pub fun: FmtGenFloat64Fn,
    pub defaultfmt: Option<String>
}

type FmtGenInt64Fn = fn(&Vec<modules::ModuleData>, std::time::Duration) -> Result<Option<i64>, String>;
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
                defaultfmt: Some($defaultfmt.to_string())
            })
		}
	};
	($char:literal, i64 $fnname:ident) => {
		utils::FormatOption {
			id: $char,
            generate: utils::FormatGenerator::OutputInt64(utils::FmtGenInt64 {
                fun: $fnname,
                defaultfmt: None
            })
		}
	};
	($char:literal, i64 $fnname:ident, $defaultfmt:literal) => {
		utils::FormatOption {
			id: $char,
            generate: utils::FormatGenerator::OutputInt64(utils::FmtGenInt64 {
                fun: $fnname,
                defaultfmt: Some($defaultfmt.to_string())
            })
		}
	};
}

macro_rules! fmtoptparam {
	($char:literal, $type:ident, $defaultval:literal) => {
		utils::FormatOptionParam {
			id: $char,
            val: FormatOptionParamVal::$type($defaultval)
		}
	};
}

enum FormatOptionParamVal {
    TypeFloat64(f64),
    TypeInt64(i64),
    TypeUsize(usize)
}

struct FormatOptionParam {
    pub id: char,
    pub val: FormatOptionParamVal
}

fn internalerrormsg() -> Result<String, String> {
    Err("Internal error".to_string())
}

fn callformatfnstr(fun: &FmtGenString, _iter: &mut CharIterator, data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<String, String> {
    /*
     * String format syntax: `%T`
     *   T = token
     */

    match fun(data, ts)? {
        Some(val) => return Ok(val),
        None => return Ok("".to_string())
    }
}

macro_rules! handlefmtopt {
    ($enumtype:ident is $type:ty, $dest:ident[$idx:ident] = $src:ident) => {
        match $src.parse::<$type>() {
            Ok(val) => { $dest[$idx].val = FormatOptionParamVal::$enumtype(val); },
            Err(_) => {}
        }
    }
}

fn setfmtoptparam(opts: &mut [FormatOptionParam], tagstr: &String) {
    let mut tagchars = tagstr.chars();

    let tag = tagchars.next().unwrap(); // We're sure that tagstr will be at least 1 char
    let contents = tagchars.as_str();

    for i in 0..opts.len() {
        if opts[i].id == tag {
            match opts[i].val {
                FormatOptionParamVal::TypeFloat64(_) => handlefmtopt!(TypeFloat64 is f64, opts[i] = contents),
                FormatOptionParamVal::TypeUsize(_) => handlefmtopt!(TypeUsize is usize, opts[i] = contents),
                FormatOptionParamVal::TypeInt64(_) => handlefmtopt!(TypeInt64 is i64, opts[i] = contents)
            }
        }
    }
}

fn parsefmtoptparams(opts: &mut [FormatOptionParam], iter: &mut CharIterator) {
    match iter.peek() {
        Some(val) => if *val != '[' {
            return;
        },
        None => return
    }

    iter.next();

    let mut currtag = String::new();
    let mut c: char;
    let mut shouldexit = false;

    loop {
        c = match iter.next() {
            Some(val) => {
                if val == ']' {
                    shouldexit = true;
                    ' '
                } else {
                    val
                }
            },
            None => {
                shouldexit = true;
                ' ' 
            }
        };

        if c != ' ' {
            currtag.push(c);
        } else if currtag.len() > 0 {
            setfmtoptparam(opts, &currtag);
            currtag = String::new();
        }

        if shouldexit {
            break;
        }
    }
}

fn callformatfnf64(fun: &FmtGenFloat64, iter: &mut CharIterator, data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<String, String> {
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

    let mut result: f64 = match (fun.fun)(data, ts)? {
        Some(val) => val,
        None => return Ok("?".to_string())
    };

    let opts: &mut [FormatOptionParam] = &mut [
        fmtoptparam!('d', TypeFloat64, 1.0),
        fmtoptparam!('p', TypeUsize, 0),
        fmtoptparam!('z', TypeUsize, 0)
    ];

    match &fun.defaultfmt {
        Some(fmt) => parsefmtoptparams(opts, &mut fmt.chars().peekable()),
        None => {}
    }

    parsefmtoptparams(opts, iter);

    let FormatOptionParamVal::TypeFloat64(divisor) = opts[0].val else {
        return internalerrormsg();
    };

    let FormatOptionParamVal::TypeUsize(decimals) = opts[1].val else {
        return internalerrormsg();
    };

    let FormatOptionParamVal::TypeUsize(zeropad) = opts[2].val else {
        return internalerrormsg();
    };

    result /= divisor;

    let resultstr = format!("{:.decimals$}", result.abs(), decimals = decimals);

    let len = match resultstr.find(|c: char| !c.is_digit(10)) {
        Some(val) => val,
        None => resultstr.len()
    };

    Ok(if len < zeropad {
        ("0".repeat(zeropad - len) + &resultstr).to_string()
    } else {
        resultstr
    })
}

fn callformatfni64(fun: &FmtGenInt64, iter: &mut CharIterator, data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<String, String> {
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

    let mut result: i64 = match (fun.fun)(data, ts)? {
        Some(val) => val,
        None => return Ok("?".to_string())
    };

    let opts: &mut [FormatOptionParam] = &mut [
        fmtoptparam!('d', TypeInt64, 1),
        fmtoptparam!('r', TypeInt64, 0),
        fmtoptparam!('z', TypeUsize, 0)
    ];

    match &fun.defaultfmt {
        Some(fmt) => parsefmtoptparams(opts, &mut fmt.chars().peekable()),
        None => {}
    }

    parsefmtoptparams(opts, iter);

    let FormatOptionParamVal::TypeInt64(divisor) = opts[0].val else {
        return internalerrormsg();
    };

    let FormatOptionParamVal::TypeInt64(moddivisor) = opts[1].val else {
        return internalerrormsg();
    };

    let FormatOptionParamVal::TypeUsize(zeropad) = opts[2].val else {
        return internalerrormsg();
    };

    result /= divisor;

    if moddivisor != 0 {
        result %= moddivisor;
    }

    let resultstr = format!("{}", result);

    let len = match resultstr.find(|c: char| !c.is_digit(10)) {
        Some(val) => val,
        None => resultstr.len()
    };

    Ok(if len < zeropad {
        ("0".repeat(zeropad - len) + &resultstr).to_string()
    } else {
        resultstr
    })
}

fn callformatfn(iter: &mut CharIterator, fmtopts: &[FormatOption], data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<String, String> {
    let default = "%".to_string();

    let c = match iter.next() {
        Some(c) => c,
        None => return Ok(default)
    };

    if c == '%' {
        return Ok(default);
    }

    for opt in fmtopts {
        if opt.id == c {
            match &opt.generate {
                FormatGenerator::OutputString(fun) => return callformatfnstr(&fun, iter, data, ts),
                FormatGenerator::OutputFloat64(fun) => return callformatfnf64(&fun, iter, data, ts),
                FormatGenerator::OutputInt64(fun) => return callformatfni64(&fun, iter, data, ts)
            }
        }
    }

    Ok(format!("%{}", c))
}

pub fn format(fmt: &String, fmtopts: &[FormatOption], data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<Option<String>, String> {
    let mut out = String::new();
    
    let mut iter = fmt.chars().peekable();

    loop {
        let c: char = match iter.next() {
            Some(c) => c,
            None => break
        };

        if c == '%' {
            out += &callformatfn(&mut iter, fmtopts, data, ts)?;
            continue;
        }

        out.push(c);
    }

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

    let opts: &[FormatOption] = &[
        fmtopt!('d', i64 parsetime, "[d86400000]"),
        fmtopt!('H', i64 parsetime, "[d3600000 r60 z2]"),
        fmtopt!('h', i64 parsetime, "[d3600000]"),
        fmtopt!('M', i64 parsetime, "[d60000 r60 z2]"),
        fmtopt!('m', i64 parsetime, "[d60000]"),
        fmtopt!('S', i64 parsetime, "[d1000 r60 z2]"),
        fmtopt!('s', i64 parsetime, "[d1000]"),
        fmtopt!('L', i64 parsetime, "[d1 r60 z2]"),
        fmtopt!('l', i64 parsetime, "[d1]"),
    ];

    format(&fmt, opts, &data, std::time::Duration::MAX)
}

