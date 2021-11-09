mod context;
mod macros;
mod run_commands;
mod trigger_commands;
mod types;
mod utils;

use crate::context::YabaiPlugin;

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
