use crate::{
	context::{window::create_windows_manager, YabaiPlugin},
	some_or_return,
	types::Window,
};

pub fn close_focused_window(plugin: &YabaiPlugin) {
	let mut wm = create_windows_manager(plugin);
	let window_to_close = some_or_return!(wm.get_focused_window());

	// Sort the windows from top to bottom
	let mut master_windows = wm.get_master_windows();
	master_windows.sort_by(|w1, w2| w1.frame.y.cmp(&w2.frame.y));
	let mut stack_windows = wm.get_stack_windows();
	stack_windows.sort_by(|w1, w2| w1.frame.y.cmp(&w2.frame.y));

	let mut window_to_focus: Option<&Window> = None;
	if wm.is_stack_window(&window_to_close) {
		// If the window is the only stack window, then focus on the master window
		if stack_windows.len() == 1 {
			window_to_focus = Some(master_windows[0]);
		}
		// Focus on the window above it, or if there is no window above it, then the window below it
		else {
			let window_position = stack_windows
				.iter()
				.position(|w| w.id == window_to_close.id);

			if let Some(position) = window_position {
				if position == 0 {
					window_to_focus = Some(stack_windows[1]);
				} else {
					window_to_focus = Some(stack_windows[position - 1]);
				}
			}
		}
	} else if wm.is_master_window(&window_to_close) {
		// If the window is the only master window and there is at least one stack window,
		// focus on the bottom stack window
		if master_windows.len() == 1 && stack_windows.len() > 0 {
			window_to_focus = Some(stack_windows[stack_windows.len() - 1]);
		}
		// Focus on the window above it, or if there is no window above it, then the window below it
		else {
			let window_position = master_windows
				.iter()
				.position(|w| w.id == window_to_close.id);

			if let Some(position) = window_position {
				if position == 0 {
					window_to_focus = Some(master_windows[1]);
				} else {
					window_to_focus = Some(master_windows[position - 1]);
				}
			}
		}
	}

	wm.run_yabai_command("-m window --close");

	if let Some(window_to_focus) = window_to_focus {
		wm.run_yabai_command(&format!("-m window --focus {}", window_to_focus.id));
	}
}
