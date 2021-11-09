use crate::context::{
	window::{create_windows_manager, UpdateWindowsProps},
	YabaiPlugin,
};

pub fn increase_master_window_count(plugin: &YabaiPlugin) {
	let wm = create_windows_manager(plugin);
	let state = plugin.read_state();
	if state.numMasterWindows[&wm.space.id] < wm.windows_data.len() - 1 {
		state.numMasterWindows[&wm.space.id] += 1;
		plugin.write_state(&state);
		log::debug!("Increasing master window count.");
	}
	wm.update_windows(UpdateWindowsProps {
		target_num_master_windows: state.numMasterWindows[&wm.space.id],
	})
}
