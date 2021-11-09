use crate::context::{
	window::{create_windows_manager, UpdateWindowsProps},
	YabaiPlugin,
};

pub fn yabai_started(plugin: &YabaiPlugin) {
	let mut wm = create_windows_manager(plugin);
	let state = plugin.read_state();
	wm.update_windows(UpdateWindowsProps {
		target_num_master_windows: state.num_master_windows[&wm.space.id],
	});
}
