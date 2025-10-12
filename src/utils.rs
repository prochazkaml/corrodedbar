use crate::formatter;
use crate::fmtopt;

pub fn readstring(path: &str) -> Result<String, String> {
	// Yes, this is just a fancy wrapper function.
	
	std::fs::read_to_string(path)
		.map_err(|e| format!("File read error: {}", e))
}

pub fn readline(path: &str) -> Result<String, String> {
	let file = readstring(path)?;

	let Some(val) = file.lines().next() else {
		Err("File parse error".to_string())?
	};

	Ok(val.to_string())
}

pub fn readlineas<T>(path: &str) -> Result<T, String>
	where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
	
	readline(path)?.split(' ').next().unwrap().parse::<T>()
		.map_err(|e| format!("Format error: {:?}", e))
}

pub fn formatduration(fmt: &str, dur: f64) -> Result<Option<String>, String> {
	let timeint = (dur * 1000.0) as i64;

	formatter::format(fmt, |tag| {
		Ok(Some(match tag {
			'd' => fmtopt!(i64 raw timeint, "[d86400000]"),
			'H' => fmtopt!(i64 raw timeint, "[d3600000 r24 z2]"),
			'h' => fmtopt!(i64 raw timeint, "[d3600000]"),
			'M' => fmtopt!(i64 raw timeint, "[d60000 r60 z2]"),
			'm' => fmtopt!(i64 raw timeint, "[d60000]"),
			'S' => fmtopt!(i64 raw timeint, "[d1000 r60 z2]"),
			's' => fmtopt!(i64 raw timeint, "[d1000]"),
			'L' => fmtopt!(i64 raw timeint, "[d1 r60 z2]"),
			'l' => fmtopt!(i64 raw timeint, "[d1]"),
			_ => return Ok(None)
		}))
	})
}

