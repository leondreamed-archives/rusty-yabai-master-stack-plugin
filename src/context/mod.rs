pub mod space;
pub mod display;
pub mod window;
pub mod state;

pub struct PluginContext {
	yabai_path: &'static str,
	debug: bool,
}

impl PluginContext {
	pub fn new() -> Self {
		PluginContext {
			yabai_path: env!("YABAI_PATH"),
			debug: option_env!("DEBUG") == Some("1"),
		}
	}

	pub fn run_yabai_command(&self, command: &str) -> String {
		let (code, output, error) = run_script!(format!("{} {}", self.yabai_path, script)).expect("Failed to run script");
		return output;
	}
}
