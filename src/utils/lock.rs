use std::fs;
use std::io::ErrorKind;

pub struct LockManager {
	pub lock_path: String,
	pub locked: bool,
}

impl LockManager {
	pub fn new(lock_path: String) -> Self {
		Self {
			lock_path,
			locked: false,
		}
	}

	pub fn release_lock(&self, force: bool) -> anyhow::Result<()> {
		if force || self.locked {
			if let Err(e) = fs::remove_dir(&self.lock_path) {
				if e.kind() != ErrorKind::NotFound {
					return Err(anyhow::Error::msg("Failed to release lock."));
				}
			}
		}
		Ok(())
	}

	pub fn acquire_lock(&self) -> anyhow::Result<()> {
		if let Err(e) = fs::create_dir(&self.lock_path) {
			if e.kind() == ErrorKind::AlreadyExists {
				return Err(anyhow::Error::msg("Failed to acquire lock."));
			}
		}
		Ok(())
	}
}
