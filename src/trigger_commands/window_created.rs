use std::env;

use crate::context::{
	window::{
		create_windows_manager, CheckValidLayoutPayload, CheckValidLayoutProps, GetWindowDataProps,
		UpdateWindowsProps,
	},
	YabaiPlugin,
};

pub fn window_created(plugin: &YabaiPlugin) {
	log::debug!("Starting to handle window_created");
	let mut wm = create_windows_manager(plugin);

	if let CheckValidLayoutPayload::Success = wm.check_valid_layout(CheckValidLayoutProps {
		target_num_master_windows: None,
	}) {
		log::debug!("Valid layout detected; no changes were made.");
	}

	let cur_num_master_windows = wm.get_master_windows().len();

	let window;
	if let Ok(process_id) = env::var("YABAI_PROCESS_ID") {
		window = wm.get_window_data(GetWindowDataProps::ProcessId(
			process_id.parse().expect("Failed to parse process ID"),
		))
	} else {
		if let Ok(window_id) = env::var("YABAI_WINDOW_ID") {
			window = wm.get_window_data(GetWindowDataProps::WindowId(
				window_id.parse().expect("Failed to parse window ID"),
			))
		} else {
			panic!("YABAI_PROCESS_ID and YABAI_WINDOW_ID not found in environment.");
		}
	}

	let state = plugin.read_state();

	if cur_num_master_windows > 1 && cur_num_master_windows <= state.num_master_windows[&wm.space.id]
	{
		// move the window to the master
		log::debug!("Moving newly created window to master.");
		wm.move_window_to_master(&window);
	}
	// if there are too many windows on the master
	else {
		log::debug!("Moving newly created window to stack.");
		wm.move_window_to_stack(&window);
	}

	wm.update_windows(UpdateWindowsProps {
		target_num_master_windows: state.num_master_windows[&wm.space.id],
	});

	log::debug!("Finished handling window_created");
}
