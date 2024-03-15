use crate::config;
use crate::modules;
use crate::utils;
use crate::fmtopt;
use crate::getdata;
use crate::configmandatory;
use crate::configoptional;

enum Data {
    DEVICECURR,
    DEVICEMAX,
    FORMAT
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    configmandatory!("_devicecurr", TypeString, data, config);
    configmandatory!("_devicemax", TypeString, data, config);
    configoptional!("_format", TypeString, "%u%%", data, config);

	Ok(data)
}

fn getvalue(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<i64>, String> {
    getdata!(dev, DEVICECURR, TypeString, data);

    let curr: i64 = utils::readlineas(dev.to_string())?;

    Ok(Some(curr))
}

fn getmaxvalue(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<i64>, String> {
    getdata!(dev, DEVICEMAX, TypeString, data);

    let max: i64 = utils::readlineas(dev.to_string())?;

    Ok(Some(max))
}

fn getvalueperc(data: &Vec<modules::ModuleData>, ts: std::time::Duration) -> Result<Option<f64>, String> {
    let curr: f64 = getvalue(data, ts)?.unwrap() as f64;
    let max: f64 = getmaxvalue(data, ts)?.unwrap() as f64;

    Ok(Some(curr / max))
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(fmt, FORMAT, TypeString, data);

    let opts: &[utils::FormatOption] = &[
        fmtopt!('c', i64 getvalue),
        fmtopt!('u', f64 getvalueperc, "[d.01]"),
        fmtopt!('m', i64 getmaxvalue),
    ];

    utils::format(fmt, opts, data, _ts)
}

