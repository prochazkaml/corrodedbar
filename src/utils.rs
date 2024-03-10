use crate::modules;

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

