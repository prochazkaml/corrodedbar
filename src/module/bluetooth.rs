use crate::config;
use crate::modules;
use crate::config_optional;

struct Bluetooth {
	enabled: String
}

impl modules::ModuleImplementation for Bluetooth {
	fn run(&mut self, _ts: std::time::Duration) -> Result<Option<String>, String> {
		let mut is_enabled = false;

		unsafe {
			let file = libc::open(c"/dev/rfkill".as_ptr(), libc::O_RDONLY);
		
			if file < 0 {
				Err("/dev/rfkill inaccessible".to_string())?
			}

			libc::fcntl(file, libc::F_SETFL, libc::O_NONBLOCK);

			loop {
				let mut event: Vec<u8> = vec![0; 8];

				let read = libc::read(file, event.as_mut_ptr() as *mut libc::c_void, 8);

				if read <= 0 && *libc::__errno_location() == libc::EAGAIN {
					break
				}

				if event[4] != 2 { // Not bluetooth
					continue
				}

				if event[6] == 0 && event[7] == 0 { // Soft & hard unblocked
					is_enabled = true;
					break
				}
			}

			libc::close(file);
		}

		Ok(is_enabled.then(|| self.enabled.to_string()))
	}
}

pub fn init(config: &Vec<config::ConfigKeyValue>) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	Ok(Box::new(Bluetooth {
		enabled: config_optional!(config, "_enabled", "Enabled".to_string())
	}))
}

