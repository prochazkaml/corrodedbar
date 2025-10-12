use crate::formatter;

type CharIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub struct FmtGenString {
	pub val: String
}

pub struct FmtGenFloat64 {
	pub val: f64,
	pub default_fmt: Option<String>
}

pub struct FmtGenInt64 {
	pub val: i64,
	pub default_fmt: Option<String>
}

pub enum FormatGenerator {
	OutputString(FmtGenString),
	OutputFloat64(FmtGenFloat64),
	OutputInt64(FmtGenInt64)
}

#[macro_export]
macro_rules! fmt_opt {
	($type:ident $val:expr, $other:expr) => {
		match $val {
			Ok(Some(val)) => Ok(Some(fmt_opt!($type raw val, $other))),
			Ok(None) => Ok(None),
			Err(val) => Err(val)
		}
	};

	($type:ident $val:expr) => {
		match $val {
			Ok(Some(val)) => Ok(Some(fmt_opt!($type raw val))),
			Ok(None) => Ok(None),
			Err(val) => Err(val)
		}
	};

	(String raw $val:expr) => {
		formatter::FormatGenerator::OutputString(formatter::FmtGenString {
			val: $val
		})
	};
	(f64 raw $val:expr) => {
		formatter::FormatGenerator::OutputFloat64(formatter::FmtGenFloat64 {
			val: $val,
			default_fmt: None
		})
	};
	(f64 raw $val:expr, $defaultfmt:literal) => {
		formatter::FormatGenerator::OutputFloat64(formatter::FmtGenFloat64 {
			val: $val,
			default_fmt: Some($defaultfmt.to_string())
		})
	};
	(i64 raw $val:expr) => {
		formatter::FormatGenerator::OutputInt64(formatter::FmtGenInt64 {
			val: $val,
			default_fmt: None
		})
	};
	(i64 raw $val:expr, $defaultfmt:literal) => {
		formatter::FormatGenerator::OutputInt64(formatter::FmtGenInt64 {
			val: $val,
			default_fmt: Some($defaultfmt.to_string())
		})
	};
}

macro_rules! fmt_opt_param {
	($char:literal, $type:ident, $defaultval:literal) => {
		formatter::FormatOptionParam {
			id: $char,
			val: FormatOptionParamVal::$type($defaultval)
		}
	};
}

enum FormatOptionParamVal {
	Float64(f64),
	Int64(i64),
	Usize(usize)
}

struct FormatOptionParam {
	pub id: char,
	pub val: FormatOptionParamVal
}

macro_rules! handle_fmt_opt {
	($enumtype:ident is $type:ty, $dest:ident = $src:ident) => {
		if let Ok(val) = $src.parse::<$type>() {
			$dest.val = FormatOptionParamVal::$enumtype(val); 
		}
	}
}

fn set_fmt_opt_param(opts: &mut [FormatOptionParam], tag: &str) {
	let mut tag_chars = tag.chars();

	let tag = tag_chars.next().unwrap(); // We're sure that tagstr will be at least 1 char
	let contents = tag_chars.as_str();

	for opt in opts {
		if opt.id == tag {
			match opt.val {
				FormatOptionParamVal::Float64(_) => handle_fmt_opt!(Float64 is f64, opt = contents),
				FormatOptionParamVal::Usize(_) => handle_fmt_opt!(Usize is usize, opt = contents),
				FormatOptionParamVal::Int64(_) => handle_fmt_opt!(Int64 is i64, opt = contents)
			}
		}
	}
}

fn parse_fmt_opt_params(opts: &mut [FormatOptionParam], iter: &mut CharIterator) {
	if iter.peek() != Some(&'[') {
		return
	};

	iter.next();

	let mut curr_tag = String::new();
	let mut should_exit = false;

	loop {
		let c = match iter.next() {
			Some(']') | None => {
				should_exit = true;
				' '
			},
			Some(val) => val
		};

		if c != ' ' {
			curr_tag.push(c);
		} else if !curr_tag.is_empty() {
			set_fmt_opt_param(opts, &curr_tag);
			curr_tag = String::new();
		}

		if should_exit {
			break
		}
	}
}

fn parse_fmt_f64(iter: &mut CharIterator, val: FmtGenFloat64) -> Result<String, String> {
	/*
	 * Float format syntax: `%T[dD pP zZ]`
	 *   T = token
	 *   D = divisor, 1 to disable (duh)
	 *     1 by default
	 *   P = number of output decimal places
	 *     0 by default
	 *   Z = minimum number of digits before the decimal point (zero-pad)
	 *     0 by default
	 *
	 *   result = fnoutput / D; rounded to P decimal places, zero-padded to Z digits
	 */

	let mut result = val.val;

	let opts: &mut [FormatOptionParam] = &mut [
		fmt_opt_param!('d', Float64, 1.0),
		fmt_opt_param!('p', Usize, 0),
		fmt_opt_param!('z', Usize, 0)
	];

	if let Some(fmt) = &val.default_fmt {
		parse_fmt_opt_params(opts, &mut fmt.chars().peekable());
	}

	parse_fmt_opt_params(opts, iter);

	let FormatOptionParamVal::Float64(divisor) = opts[0].val else { panic!() };
	let FormatOptionParamVal::Usize(decimals) = opts[1].val else { panic!() };
	let FormatOptionParamVal::Usize(zeropad) = opts[2].val else { panic!() };

	result /= divisor;

	let mut result_str = format!("{:.decimals$}", result.abs(), decimals = decimals);

	let len = result_str.find(|c: char| !c.is_ascii_digit()).unwrap_or(result_str.len());

	if len < zeropad {
		result_str = "0".repeat(zeropad - len) + &result_str
	}

	Ok(result_str)
}

fn parse_fmt_i64(iter: &mut CharIterator, val: FmtGenInt64) -> Result<String, String> {
	/*
	 * Float format syntax: `%T[dD rR zZ]`
	 *   T = token
	 *   D = divisor, 1 to disable (duh)
	 *     1 by default
	 *   R = divisor for remainder calculation, 0 to disable
	 *     0 by default
	 *   Z = minimum number of digits before the decimal point (zero-pad)
	 *     0 by default
	 *
	 *   result = (fnoutput / D) % R; zero-padded to Z digits
	 */

	let opts: &mut [FormatOptionParam] = &mut [
		fmt_opt_param!('d', Int64, 1),
		fmt_opt_param!('r', Int64, 0),
		fmt_opt_param!('z', Usize, 0)
	];

	let mut result = val.val;

	if let Some(fmt) = &val.default_fmt {
		parse_fmt_opt_params(opts, &mut fmt.chars().peekable());
	}

	parse_fmt_opt_params(opts, iter);

	let FormatOptionParamVal::Int64(divisor) = opts[0].val else { panic!() };
	let FormatOptionParamVal::Int64(mod_divisor) = opts[1].val else { panic!() };
	let FormatOptionParamVal::Usize(zeropad) = opts[2].val else { panic!() };

	result /= divisor;

	if mod_divisor != 0 {
		result %= mod_divisor;
	}

	let mut result_str = result.to_string();

	let len = result_str.find(|c: char| !c.is_ascii_digit()).unwrap_or(result_str.len());

	if len < zeropad {
		result_str = "0".repeat(zeropad - len) + &result_str
	}

	Ok(result_str)
}

fn parse_fmt_val(iter: &mut CharIterator, val: FormatGenerator) -> Result<String, String> {
	match val {
		FormatGenerator::OutputString(val) => Ok(val.val),
		FormatGenerator::OutputFloat64(val) => parse_fmt_f64(iter, val),
		FormatGenerator::OutputInt64(val) => parse_fmt_i64(iter, val)
	}
}

pub fn format<F>(fmt: &str, f: F) -> Result<Option<String>, String> where F: Fn(char) -> Result<Option<FormatGenerator>, String> {
	let mut out = String::new();
	
	let mut iter = fmt.chars().peekable();

	loop {
		let Some(c) = iter.next() else {
			break
		};

		if c == '%' {
			let Some(tag) = iter.next() else {
				break
			};

			match f(tag)? {
				Some(val) => out += &parse_fmt_val(&mut iter, val)?,
				None => out.push(tag)
			};

			continue
		}

		out.push(c);
	}

	Ok(Some(out))
}

