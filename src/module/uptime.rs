use crate::config;
use crate::modules;
use crate::utils;
use crate::fmtopt;
use crate::getdata;
use crate::configoptional;

enum Data {
    FORMAT,
    SUBUPTIME
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    configoptional!("_format", TypeString, "%dd %Hh %Mm", data, config);

	Ok(data)
}

macro_rules! genuptimefun {
	($fnname:ident, $cap: literal, $div:literal, $mod:literal, $decimals:literal) => {
        pub fn $fnname(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
            getdata!(uptime, SUBUPTIME, TypeFloat64, data);

            let uptimeint = (uptime * 1000.0) as u64;

            Ok(Some(if $cap {
                format!(concat!("{:0>", $decimals, "}"), uptimeint / $div % $mod)
            } else {
                format!("{}", uptimeint / $div)
            }))
        }
	}
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

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    getdata!(fmt, FORMAT, TypeString, data);

    let uptime: f64 = utils::readlineas("/proc/uptime".to_string())?;

    let mut subdata = data.clone();
    subdata.push(modules::ModuleData::TypeFloat64(uptime));

    let opts: &[utils::FormatOption] = &[
        fmtopt!('d', getday),
        fmtopt!('H', gethourcapped),
        fmtopt!('h', gethour),
        fmtopt!('M', getminutecapped),
        fmtopt!('m', getminute),
        fmtopt!('S', getsecondcapped),
        fmtopt!('s', getsecond),
        fmtopt!('L', getmilliscapped),
        fmtopt!('l', getmillis)
    ];

    utils::format(fmt, opts, &subdata, _ts)
}

