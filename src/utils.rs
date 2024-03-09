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

pub fn readline(path: String) -> Result<String, String> {
    let file = match std::fs::read_to_string(path) {
        Ok(val) => val,
        Err(errmsg) => { return Err(format!("File read error: {}", errmsg)); }
    };

    match file.lines().next() {
        Some(val) => Ok(val.to_string()),
        None => { return Err("File parse error".to_string()); }
    }
}
