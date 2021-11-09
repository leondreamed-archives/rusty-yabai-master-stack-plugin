use std::fs;
use std::io::ErrorKind;

pub struct LockManager {
	pub lock_path: String,
}

impl LockManager {
	pub fn new(lock_path: String) -> Self {
		Self { lock_path }
	}

	pub fn release_lock(&self) {
		if let Err(e) = fs::remove_dir(&self.lock_path) {
			if e.kind() != ErrorKind::NotFound {
				panic!("Failed to release lock.");
			}
		}
	}

	pub fn acquire_lock(&self) {
		if let Err(e) = fs::create_dir(&self.lock_path) {
			if e.kind() == ErrorKind::AlreadyExists {
				panic!("Could not acquire lock.");
			}
		}
	}
}
