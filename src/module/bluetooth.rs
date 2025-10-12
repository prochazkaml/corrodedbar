use crate::modules;

use toml::Table;

#[derive(serde::Deserialize)]
struct Bluetooth {
	#[serde(default = "default_enabled")]
	enabled: String
}

fn default_enabled() -> String { "enabled".to_string() }

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

pub fn init(config: Table) -> Result<Box<dyn modules::ModuleImplementation>, String> {
	let new: Bluetooth = Table::try_into(config).map_err(|err| format!("Error reading `bluetooth` config: {err}"))?;

	Ok(Box::new(new))
}

