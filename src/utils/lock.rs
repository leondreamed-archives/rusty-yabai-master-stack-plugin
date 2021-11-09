use std::fs;
use std::io::ErrorKind;

pub struct LockManager {
	lock_path: String,
	locked: bool,
}

impl LockManager {
	pub fn new(lock_path: String) -> Self {
		Self {
			lock_path,
			locked: false,
		}
	}

	pub fn release_lock(&self, force: bool) {
		if force || self.locked {
			if let Err(e) = fs::remove_dir(self.lock_path) {
				if e.kind() != ErrorKind::NotFound {
					panic!("Failed to release lock.");
				}
			}
			self.locked = false;
		}
	}

	pub fn acquire_lock(&self, lock_path: String) {
		if let Err(e) = fs::create_dir(lock_path) {
			if e.kind() == ErrorKind::AlreadyExists {
				panic!("Could not acquire lock.");
			}
		}
		self.locked = true;
	}
}
