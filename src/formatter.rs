use crate::formatter;
use crate::modules;

type CharIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub struct FmtGenString {
	pub val: String
}

pub struct FmtGenFloat64 {
	pub val: f64,
	pub defaultfmt: Option<String>
}

pub struct FmtGenInt64 {
	pub val: i64,
	pub defaultfmt: Option<String>
}

pub enum FormatGenerator {
	OutputString(FmtGenString),
	OutputFloat64(FmtGenFloat64),
	OutputInt64(FmtGenInt64)
}

#[macro_export]
macro_rules! fmtopt {
	($type:ident $val:expr, $other:expr) => {
		match $val {
			Ok(val) => match val {
				Some(val) => Ok(Some(fmtopt!($type enum val, $other))),
				None => Ok(None)
			},
			Err(val) => Err(val)
		}
	};

	($type:ident $val:expr) => {
		match $val {
			Ok(val) => match val {
				Some(val) => Ok(Some(fmtopt!($type enum val))),
				None => Ok(None)
			},
			Err(val) => Err(val)
		}
	};

	(String enum $val:expr) => {
		formatter::FormatGenerator::OutputString(formatter::FmtGenString {
			val: $val
		})
	};
	(f64 enum $val:expr) => {
		formatter::FormatGenerator::OutputFloat64(formatter::FmtGenFloat64 {
			val: $val,
			defaultfmt: None
		})
	};
	(f64 enum $val:expr, $defaultfmt:literal) => {
		formatter::FormatGenerator::OutputFloat64(formatter::FmtGenFloat64 {
			val: $val,
			defaultfmt: Some($defaultfmt.to_string())
		})
	};
	(i64 enum $val:expr) => {
		formatter::FormatGenerator::OutputInt64(formatter::FmtGenInt64 {
			val: $val,
			defaultfmt: None
		})
	};
	(i64 enum $val:expr, $defaultfmt:literal) => {
		formatter::FormatGenerator::OutputInt64(formatter::FmtGenInt64 {
			val: $val,
			defaultfmt: Some($defaultfmt.to_string())
		})
	};
}

macro_rules! fmtoptparam {
	($char:literal, $type:ident, $defaultval:literal) => {
		formatter::FormatOptionParam {
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

fn parsefmtf64(iter: &mut CharIterator, val: FmtGenFloat64) -> Result<String, String> {
	/*
	 * Float format syntax: `%T[dD pP zZ]`
	 *   T = token
	 *   D = divisor, 1 to disable (duh)
	 *	   1 by default
	 *   P = number of output decimal places
	 *	   0 by default
	 *   Z = minimum number of digits before the decimal point (zero-pad)
	 *	   0 by default
	 *
	 *   result = fnoutput / D; rounded to P decimal places, zero-padded to Z digits
	 */

	let mut result = val.val;

	let opts: &mut [FormatOptionParam] = &mut [
		fmtoptparam!('d', TypeFloat64, 1.0),
		fmtoptparam!('p', TypeUsize, 0),
		fmtoptparam!('z', TypeUsize, 0)
	];

	match &val.defaultfmt {
		Some(fmt) => parsefmtoptparams(opts, &mut fmt.chars().peekable()),
		None => {}
	}

	parsefmtoptparams(opts, iter);

	let FormatOptionParamVal::TypeFloat64(divisor) = opts[0].val else {
		return Err(modules::internalerrormsg());
	};

	let FormatOptionParamVal::TypeUsize(decimals) = opts[1].val else {
		return Err(modules::internalerrormsg());
	};

	let FormatOptionParamVal::TypeUsize(zeropad) = opts[2].val else {
		return Err(modules::internalerrormsg());
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

fn parsefmti64(iter: &mut CharIterator, val: FmtGenInt64) -> Result<String, String> {
	/*
	 * Float format syntax: `%T[dD rR zZ]`
	 *   T = token
	 *   D = divisor, 1 to disable (duh)
	 *	   1 by default
	 *   R = divisor for remainder calculation, 0 to disable
	 *	   0 by default
	 *   Z = minimum number of digits before the decimal point (zero-pad)
	 *	   0 by default
	 *
	 *   result = (fnoutput / D) % R; zero-padded to Z digits
	 */

	let opts: &mut [FormatOptionParam] = &mut [
		fmtoptparam!('d', TypeInt64, 1),
		fmtoptparam!('r', TypeInt64, 0),
		fmtoptparam!('z', TypeUsize, 0)
	];

	let mut result = val.val;

	match &val.defaultfmt {
		Some(fmt) => parsefmtoptparams(opts, &mut fmt.chars().peekable()),
		None => {}
	}

	parsefmtoptparams(opts, iter);

	let FormatOptionParamVal::TypeInt64(divisor) = opts[0].val else {
		return Err(modules::internalerrormsg());
	};

	let FormatOptionParamVal::TypeInt64(moddivisor) = opts[1].val else {
		return Err(modules::internalerrormsg());
	};

	let FormatOptionParamVal::TypeUsize(zeropad) = opts[2].val else {
		return Err(modules::internalerrormsg());
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

fn parsefmtval(iter: &mut CharIterator, val: FormatGenerator) -> Result<String, String> {
	match val {
		FormatGenerator::OutputString(val) => Ok(val.val),
		FormatGenerator::OutputFloat64(val) => parsefmtf64(iter, val),
		FormatGenerator::OutputInt64(val) => parsefmti64(iter, val)
	}
}
//pub fn run_thread<F>(ctx: &mut AppContext, f: F) -> Result<(), Value> where F: FnOnce() -> Result<(), Value> + std::marker::Send + 'static {

pub fn format<F>(fmt: &String, f: F) -> Result<Option<String>, String> where F: Fn(char) -> Result<Option<FormatGenerator>, String> {
	let mut out = String::new();
	
	let mut iter = fmt.chars().peekable();

	loop {
		let c: char = match iter.next() {
			Some(c) => c,
			None => break
		};

		if c == '%' {
			let tag = match iter.next() {
				Some(val) => val,
				None => break
			};

			match f(tag)? {
				Some(val) => out += &parsefmtval(&mut iter, val)?,
				None => out.push(tag)
			};

			continue;
		}

		out.push(c);
	}

	return Ok(Some(out));
}

