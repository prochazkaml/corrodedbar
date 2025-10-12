use crate::formatter;
use crate::fmt_opt;

pub fn read_string(path: &str) -> Result<String, String> {
	// Yes, this is just a fancy wrapper function.
	
	std::fs::read_to_string(path)
		.map_err(|e| format!("File read error: {}", e))
}

pub fn read_line(path: &str) -> Result<String, String> {
	let file = read_string(path)?;

	let Some(val) = file.lines().next() else {
		Err("File parse error".to_string())?
	};

	Ok(val.to_string())
}

pub fn read_line_as<T>(path: &str) -> Result<T, String>
	where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
	
	read_line(path)?.split(' ').next().unwrap().parse::<T>()
		.map_err(|e| format!("Format error: {:?}", e))
}

pub fn format_duration(fmt: &str, dur: f64) -> Result<Option<String>, String> {
	let time_int = (dur * 1000.0) as i64;

	formatter::format(fmt, |tag| {
		Ok(Some(match tag {
			'd' => fmt_opt!(i64 raw time_int, "[d86400000]"),
			'H' => fmt_opt!(i64 raw time_int, "[d3600000 r24 z2]"),
			'h' => fmt_opt!(i64 raw time_int, "[d3600000]"),
			'M' => fmt_opt!(i64 raw time_int, "[d60000 r60 z2]"),
			'm' => fmt_opt!(i64 raw time_int, "[d60000]"),
			'S' => fmt_opt!(i64 raw time_int, "[d1000 r60 z2]"),
			's' => fmt_opt!(i64 raw time_int, "[d1000]"),
			'L' => fmt_opt!(i64 raw time_int, "[d1 r60 z2]"),
			'l' => fmt_opt!(i64 raw time_int, "[d1]"),
			_ => return Ok(None)
		}))
	})
}

