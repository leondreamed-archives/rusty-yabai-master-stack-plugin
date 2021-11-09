use crate::context::{
	window::{create_windows_manager, UpdateWindowsProps},
	YabaiPlugin,
};

pub fn decrease_master_window_count(plugin: &YabaiPlugin) {
	let wm = create_windows_manager(plugin);
	let mut state = plugin.read_state();
	if state.num_master_windows[&wm.space.id] > 1 {
		*state.num_master_windows.get_mut(&wm.space.id).unwrap() -= 1;
		plugin.write_state(&state);
		log::debug!("Decreasing master window count.");
	}
	wm.update_windows(UpdateWindowsProps {
		target_num_master_windows: state.num_master_windows[&wm.space.id],
	})
}
