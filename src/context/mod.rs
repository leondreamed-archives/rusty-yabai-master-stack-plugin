pub mod display;
pub mod space;
pub mod state;
pub mod window;

use run_script::run_script;

pub struct YabaiPlugin {
	yabai_path: &'static str,
	debug: bool,
}

impl YabaiPlugin {
	pub fn new() -> Self {
		YabaiPlugin {
			yabai_path: env!("YABAI_PATH"),
			debug: option_env!("DEBUG") == Some("1"),
		}
	}

	pub fn run_yabai_command(&self, command: &str) -> String {
		let (code, output, error) =
			run_script!(format!("{} {}", self.yabai_path, command)).expect("Failed to run script");
		return output;
	}
}
