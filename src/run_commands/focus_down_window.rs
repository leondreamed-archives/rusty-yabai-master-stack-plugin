use crate::context::{window::create_windows_manager, YabaiPlugin};

pub fn focus_down_window(plugin: &YabaiPlugin) {
	let mut wm = create_windows_manager(plugin);
	if let Some(focused_window) = wm.get_focused_window() {
		if wm.is_master_window(&focused_window)
			&& wm.is_bottom_window(&wm.get_master_windows(), &focused_window)
		{
			// Focus on the top stack window
			if let Some(window_to_focus) = wm
				.get_top_stack_window()
				.or_else(|| wm.get_top_master_window())
			{
				log::debug!("Focusing on the window {}", window_to_focus.app);
				wm.run_yabai_command(&format!("-m window --focus {}", window_to_focus.id));
			}
		} else if wm.is_stack_window(focused_window)
			&& wm.is_bottom_window(&wm.get_stack_windows(), focused_window)
		{
			// Focus on the top master window
			if let Some(window_to_focus) = wm.get_top_master_window() {
				log::debug!("Focusing on the window {}", window_to_focus.app);
				wm.run_yabai_command(&format!("-m window --focus {}", window_to_focus.id));
			}
		}
		// Otherwise, just focus south
		else {
			wm.run_yabai_command("-m window --focus south");
		}
	} else {
		wm.run_yabai_command("-m window --focus first");
	}
}
