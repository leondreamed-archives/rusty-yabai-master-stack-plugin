mod context;
mod macros;
mod run_commands;
mod trigger_commands;
mod types;
mod utils;

use std::sync::{Arc, Mutex};

use crate::{
	context::YabaiPlugin,
	run_commands::{
		close_focused_window, decrease_master_window_count, focus_down_window, focus_up_window,
		increase_master_window_count,
	},
	trigger_commands::{window_created, window_moved, yabai_started},
	utils::lock::LockManager,
};

fn main() {
	std::env::set_var("RUST_LOG", "debug");
	env_logger::init();

	let plugin_lock_file_path =
		"/Users/leonzalion/code/rusty-yabai-master-stack-plugin/plugin.lock".to_string();

	let plugin = YabaiPlugin::new();
	let command_type = std::env::args().nth(1).expect("No command type given");
	let command_value = std::env::args().nth(2).expect("No command value given");
	let lock_manager = Arc::new(Mutex::new(LockManager::new(
		plugin_lock_file_path.to_string(),
	)));

	let panic_lock_manager = lock_manager.clone();
	std::panic::set_hook(Box::new(move |e| {
		log::debug!("{:?}", e);
		panic_lock_manager
			.lock()
			.expect("Failed to get lock.")
			.release_lock(false)
			.expect("Failed to release lock.");
	}));

	if command_type == "trigger" && command_value == "yabai-started" {
		lock_manager
			.lock()
			.expect("Failed to lock mutex")
			.release_lock(true)
			.expect("Failed to release lock");
	}
	lock_manager
		.lock()
		.expect("Failed to lock mutex")
		.acquire_lock()
		.expect("Failed to acquire lock");

	match command_type.as_str() {
		"run" => match command_value.as_str() {
			"close-focused-window" => close_focused_window(&plugin),
			"decrease-master-window-count" => decrease_master_window_count(&plugin),
			"focus-down-window" => focus_down_window(&plugin),
			"focus-up-window" => focus_up_window(&plugin),
			"increase-master-window-count" => increase_master_window_count(&plugin),
			_ => panic!("Unrecognized run command"),
		},
		"trigger" => match command_value.as_str() {
			"yabai-started" => yabai_started(&plugin),
			"window-created" => window_created(&plugin),
			"window-moved" => window_moved(&plugin),
			_ => panic!("Unrecognized trigger command"),
		},
		_ => panic!("Unrecognized command type {}", command_type),
	}

	lock_manager
		.lock()
		.expect("Failed to get mutex lock")
		.release_lock(false)
		.expect("Failed to release lock.");
}
