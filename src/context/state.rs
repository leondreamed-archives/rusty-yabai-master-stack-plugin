use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use super::YabaiPlugin;

#[derive(Deserialize, Serialize)]
pub struct State {
	pub num_master_windows: HashMap<usize, usize>,
}

impl State {
	pub fn default(context: &YabaiPlugin) -> Self {
		let spaces = context.get_spaces();
		let mut num_master_windows = HashMap::new();
		for space in spaces {
			num_master_windows.insert(space.id, 1);
		}

		Self { num_master_windows }
	}
}

impl YabaiPlugin {
	pub fn read_state(&self) -> State {
		// If the state doesn't exist, create it
		let state = match fs::read_to_string("state.json") {
			Ok(state_str) => serde_json::from_str(&state_str).expect("Failed to parse state.json"),
			Err(_) => State::default(self),
		};

		state
	}
	pub fn write_state(&self, state: &State) {
		fs::write(
			"state.json",
			serde_json::to_string(state).expect("Failed to stringify state."),
		)
		.expect("Failed to write state");
	}
}
