use crate::{context::PluginContext, types::Space};

impl PluginContext {
	pub fn get_spaces(&self) -> Vec<Space> {
		let spaces = self.run_yabai_command("-m query --spaces");
		serde_json::from_str(&spaces).expect("Failed to parse spaces")
	}

	pub fn get_focused_space(&self) -> Space {
		let space = self.run_yabai_command("-m query --spaces --space");
		serde_json::from_str(&space).expect("Failed to parse space")
	}
}