use crate::config;
use crate::modules;
use crate::getdata;
use crate::configoptional;

enum Data {
    ENABLEDSTRING
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Vec<modules::ModuleData>, String> {
	let mut data: Vec<modules::ModuleData> = Vec::new();

    configoptional!("_enabled", TypeString, "Enabled", data, config);

	Ok(data)
}

pub fn run(data: &Vec<modules::ModuleData>, _ts: std::time::Duration) -> Result<Option<String>, String> {
    let mut isenabled = false;

    unsafe {
        let file = libc::open("/dev/rfkill\0".as_ptr() as *const libc::c_char, libc::O_RDONLY);
    
        if file < 0 {
            return Err("/dev/rfkill inaccessible".to_string());
        }

        libc::fcntl(file, libc::F_SETFL, libc::O_NONBLOCK);

        loop {
            let mut event: Vec<u8> = vec![0; 8];

            let read = libc::read(file, event.as_mut_ptr() as *mut libc::c_void, 8);

            if read <= 0 && *libc::__errno_location() == libc::EAGAIN {
                break;
            }

            if event[4] != 2 { // Not bluetooth
                continue;
            }

            if event[6] == 0 && event[7] == 0 { // Soft & hard unblocked
                isenabled = true;
                break;
            }
        }

        libc::close(file);
    }

    getdata!(enstr, ENABLEDSTRING, TypeString, data);

    return if isenabled {
        Ok(Some(enstr.to_string()))
    } else {
        Ok(None)
    }
}

