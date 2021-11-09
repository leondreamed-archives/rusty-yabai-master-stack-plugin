pub mod display;
pub mod space;
pub mod state;
pub mod window;

use run_script::run_script;

pub struct YabaiPlugin {
	pub yabai_path: &'static str,
	pub debug: bool,
}

use load_dotenv::load_dotenv;

load_dotenv!();

impl YabaiPlugin {
	pub fn new() -> Self {
		YabaiPlugin {
			yabai_path: env!("YABAI_PATH"),
			debug: option_env!("DEBUG") == Some("1"),
		}
	}

	pub fn run_yabai_command(&self, command: &str) -> String {
		let (_code, output, _error) =
			run_script!(format!("{} {}", self.yabai_path, command)).expect("Failed to run script");
		output
	}
}
