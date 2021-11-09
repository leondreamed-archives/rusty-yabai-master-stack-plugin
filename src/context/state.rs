use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::YabaiPlugin;

#[derive(Deserialize, Serialize)]
pub struct State {
	pub numMasterWindows: HashMap<String, usize>
}

impl State {
	pub fn default(context: &YabaiPlugin) -> Self {
		let spaces = context.get_spaces();
		let numMasterWindows = HashMap::new();
		for space in spaces {
			numMasterWindows.insert(space.id, 1);
		}

		Self {
			numMasterWindows,
		}
	}
}

impl YabaiPlugin {
	pub fn read_state(&self) -> State {
		// If the state doesn't exist, create it
		let state = match fs::read_to_string("state.json") {
			Ok(state_str) => serde_json::from_str(&state_str).expect("Failed to parse state.json"),
			Err(_) => State::default(self)
		};

		state
	}
	pub fn write_state(&self, state: &State) {
		fs::write("state.json", serde_json::to_string(state).expect("Failed to stringify state.")).expect("Failed to write state");
	}
}

