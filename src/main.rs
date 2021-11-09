mod context;
mod run_commands;
mod trigger_commands;
mod macros;
mod types;
mod utils;

use load_dotenv::try_load_dotenv;

use crate::context::YabaiPlugin;

try_load_dotenv!();

fn main() {
	let yabai_plugin = YabaiPlugin::new();

	let command_type = std::env::args().nth(1).expect("No command type given");
	let command_value = std::env::args().nth(2).expect("No command value given");

	match command_type.as_str() {
		"run" => match command_value.as_str() {
			"close-focused-window" => {}
			"decrease-master-window-count" => {}
			"focus-down-window" => {}
			"focus-up-window" => {}
			"increase-master-window-count" => {}
			"open-new-window" => {}
		},
		"trigger" => match command_value.as_str() {
			"yabai-started" => {}
			"window-created" => {}
			"window-moved" => {}
		},
		_ => panic!("Unrecognized command type {}", command_type),
	}
}
