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
