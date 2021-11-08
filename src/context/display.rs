use crate::{context::PluginContext, types::Display};

impl PluginContext {
	pub fn get_displays(&self) -> Vec<Display> {
		let displays = self.run_yabai_command("-m query --displays");
		serde_json::from_str(&displays).expect("Failed to parse display ")
	}
	pub fn get_focused_display(&self) -> Display {
		let display = self.run_yabai_command("-m query --displays --display");
		serde_json::from_str(&display).expect("Failed to parse display")
	}
}