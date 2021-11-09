use crate::{
	context::YabaiPlugin,
	some_or_return,
	types::{Display, Space, Window},
};

use super::state::State;

mod check_valid_layout;
pub use check_valid_layout::*;

mod update_windows;
pub use update_windows::*;

struct WindowsManager<'p> {
	pub display: Display,
	pub space: Space,
	pub expected_current_num_master_windows: usize,
	pub plugin: &'p YabaiPlugin,
	pub windows_data: Vec<Window>,
}

pub fn create_windows_manager(plugin: &YabaiPlugin) -> WindowsManager {
	let mut state = plugin.read_state();
	let display = plugin.get_focused_display();

	let space = plugin.get_focused_space();

	let wm = WindowsManager {
		display,
		space,
		plugin,
		expected_current_num_master_windows: state.numMasterWindows[&space.id],
		windows_data: vec![],
	};

	wm.initialize();
	wm.validate_state(&mut state);

	wm
}

pub enum GetWindowDataProps {
	ProcessId(usize),
	WindowId(usize),
}

impl WindowsManager<'_> {
	pub fn get_windows_data(&self) -> Vec<Window> {
		let output = self.plugin.run_yabai_command("-m query --windows");
		let windowsData: Vec<Window> =
			serde_json::from_str(&output).expect("Failed to parse windows");
		windowsData.into_iter().filter(|window| {
			if window.floating != 0
				|| window.display != self.display.index
				|| window.space != self.space.index
			{
				return false;
			}

			if window.minimized == 1 {
				return false;
			}

			return true;
		});

		windowsData
	}

	pub fn validate_state(&self, state: &mut State) {
		if self.windows_data.len() < self.expected_current_num_master_windows {
			self.expected_current_num_master_windows = self.windows_data.len();
			state.numMasterWindows[&self.space.id] = self.windows_data.len();
		}

		if state.numMasterWindows[&self.space.id] <= 0 {
			state.numMasterWindows[&self.space.id] = 1;
		}

		self.plugin.write_state(state);
	}

	pub fn initialize(&self) {
		self.windows_data = self.get_windows_data();
	}

	pub fn refresh_windows_data(&self) {
		let new_windows_data = self.get_windows_data();
		self.windows_data = new_windows_data;
	}

	pub fn get_updated_window_data(&self, window: &Window) -> Option<&Window> {
		self.windows_data.iter().find(|win| window.id == win.id)
	}

	pub fn run_yabai_command(&self, command: &str) -> String {
		let output = self.plugin.run_yabai_command(command);
		self.refresh_windows_data();
		return output;
	}

	pub fn get_window_data(&self, props: GetWindowDataProps) -> Window {
		let windows_iterator = self.windows_data.into_iter();
		match props {
			GetWindowDataProps::ProcessId(process_id) => windows_iterator
				.find(|window| window.pid == process_id)
				.expect(&format!("Window with pid {} not found.", process_id)),
			GetWindowDataProps::WindowId(window_id) => windows_iterator
				.find(|window| window.id == window_id)
				.expect(&format!("Window with id {} not found", window_id)),
		}
	}

	pub fn get_focused_window(&self) -> Option<&Window> {
		self.windows_data.iter().find(|w| w.focused == 1)
	}

	/**
	 * There is always a line dividing the master windows from the secondary windows. To find this line,
		* we use two master observations:
		* 1. The top-right window is always on the right side of the dividing line.
		* 2. If there is more than one master window, the dividing line must cross the left side of two
		* windows
		* Using these observations, we can loop through the windows in descending x-coordinate starting from the top-right window
		* and for each pair of windows that share x-coordinates, we check if the numMasterWindows is less
		* than the number of windows we've iterated through, and if so, return the x-coordinate of the currently
		* processed window
		*/
	pub fn get_dividing_line_x_coordinate(&self) -> usize {
		let top_right_window = self.get_top_right_window().unwrap_or_else(|| {
			panic!("get_dividing_line_x_coordinate() was called when there are no windows.");
		});

		log::debug!("Top-right window: {}", top_right_window.app);

		if self.expected_current_num_master_windows == 1 {
			return top_right_window.frame.x;
		}

		let non_stack_windows: Vec<&Window> = self
			.windows_data
			.iter()
			.filter(|window| !self.is_stack_window(window))
			.collect();

		// Get all the non-stack windows to the left of the top-right window sorted by x coordinate
		let mut eligible_windows = non_stack_windows
			.into_iter()
			.filter(|window| window.frame.x <= top_right_window.frame.x)
			.collect::<Vec<&Window>>();

		// Sort the windows by descending order of x-coordinate
		eligible_windows.sort_by(|window1, window2| window2.frame.x.cmp(&window1.frame.x));

		let num_windows_to_right_of_top_right_window =
			non_stack_windows.len() - eligible_windows.len();

		// If there are enough windows that are to the equal/to the right of the top-right window, then return
		// the top-right window's x-coordinate
		if num_windows_to_right_of_top_right_window >= self.expected_current_num_master_windows {
			return top_right_window.frame.x;
		}

		// Otherwise, iterate through the eligible windows in order and find pairs of windows that are on top of
		// each other
		for i in 0..eligible_windows.len() - 1 {
			let cur_window = eligible_windows[i];
			let next_window = eligible_windows[i + 1];
			if cur_window.frame.x == next_window.frame.x
				&& num_windows_to_right_of_top_right_window + i + 2
					>= self.expected_current_num_master_windows
			{
				return cur_window.frame.x;
			}
		}

		// If a pair of windows could not be found (which means all the windows are side-by-side), just
		// return the top-right window's x-coordinate
		return top_right_window.frame.x;
	}

	/**
	 * The top-left window is the window with the lowest y-coordinate and the lowest x-coordinate.
	 */
	pub fn get_top_left_window(&self) -> &Window {
		let left_windows: Vec<&Window> = self
			.windows_data
			.iter()
			.filter(|window| self.is_window_touching_left_edge(window))
			.collect();

		let top_left_window = left_windows[0];
		for window in left_windows {
			if window.frame.y <= top_left_window.frame.y {
				top_left_window = window;
			}
		}

		top_left_window
	}

	/*
	 * The top-right window is the rightmost window with the lowest y-coordinate.
	 */
	pub fn get_top_right_window(&self) -> Option<&Window> {
		if self.windows_data.len() == 0 {
			return None;
		}

		let lowest_y_coordinate = self.windows_data[0].frame.y;

		for window in &self.windows_data {
			if window.frame.y < lowest_y_coordinate {
				lowest_y_coordinate = window.frame.y;
			}
		}

		let top_windows: Vec<&Window> = self
			.windows_data
			.iter()
			.filter(|window| window.frame.y == lowest_y_coordinate)
			.collect();

		let top_right_window = top_windows[0];
		for window in top_windows {
			if window.frame.x > top_right_window.frame.x {
				top_right_window = window;
			}
		}

		Some(top_right_window)
	}

	pub fn get_widest_stack_window(&self) -> Option<&Window> {
		let widest_stack_window: Option<&Window> = None;
		for window in self.get_stack_windows() {
			match widest_stack_window {
				None => widest_stack_window = Some(window),
				Some(widest_window) => {
					if window.frame.w > widest_window.frame.w {
						widest_stack_window = Some(window)
					}
				}
			}
		}

		widest_stack_window
	}

	pub fn get_widest_master_window(&self) -> Option<&Window> {
		let widest_master_window: Option<&Window> = None;

		for window in self.get_master_windows() {
			match widest_master_window {
				None => widest_master_window = Some(window),
				Some(widest_window) => {
					if window.frame.w > widest_window.frame.w {
						widest_master_window = Some(window);
					}
				}
			}
		}

		widest_master_window
	}

	// In the event that the windows get badly rearranged and all the windows span the entire width of
	// the screen, split the top-right window vertically and then move the windows into the split
	pub fn create_stack(&self) {
		log::debug!("Creating stack...");
		let top_right_window = some_or_return!(self.get_top_right_window());
		log::debug!("Top-right window: {}", top_right_window.app);

		if top_right_window.split == "horizontal" {
			self.plugin
				.run_yabai_command(&format!("-m window {} --toggle split", top_right_window.id));
		}

		self.columnize_stack_windows();
	}

	/**
	 * If the top-right window has a x-coordinate of 0, or if the stack dividing
	 * line is equal to 0, then the stack does not exist
	 */
	pub fn does_stack_exist(&self) -> bool {
		let top_right_window = self.get_top_right_window();
		match top_right_window {
			Some(window) => window.frame.x != 0,
			None => false,
		}
	}

	/**
	 * Turns the stack into a column by making sure the split direction of all the stack windows
	 * is horizontal
	 */
	pub fn columnize_stack_windows(&self) {
		// In this case, we want to columnize all the windows to the left of the dividing line
		let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate();

		let stack_windows: Vec<&Window> = self
			.windows_data
			.iter()
			.filter(|window| window.frame.x < dividing_line_x_coordinate)
			.collect();

		if stack_windows.len() > 1 {
			for stack_window in stack_windows {
				if let Some(window) = self.get_updated_window_data(stack_window) {
					if window.split == "vertical" {
						self.run_yabai_command(&format!("-m window {} --toggle split", window.id));
					}
				}
			}
		}
	}

	pub fn move_window_to_stack(&self, window: &Window) {
		log::debug!("Moving window {} to stack.", window.app);

		self.columnize_stack_windows();
		let window = some_or_return!(self.get_updated_window_data(window));

		// Don't do anything if the window is already a stack window
		if self.is_stack_window(window) {
			return;
		}

		// Use a small heuristic that helps prevent "glitchy" window rearrangements
		self.run_yabai_command(&format!("-m window {} --warp west", window.id));

		if self.windows_data.len() == 2 {
			if window.split == "horizontal" {
				self.run_yabai_command(&format!("-m window {} --toggle split", window.id));
			}

			return;
		}

		// Find a window that's touching the left side of the screen
		let stack_window = some_or_return!(self.get_widest_stack_window());

		if stack_window.id == window.id {
			return;
		}

		self.run_yabai_command(&format!(
			"-m window {} --warp {}",
			window.id, stack_window.id
		));
		let window = some_or_return!(self.get_updated_window_data(window));

		if self.windows_data.len() == 2 {
			if window.split == "horizontal" {
				self.run_yabai_command(&format!("-m window {} --toggle split", window.id));
			}
		} else {
			if window.split == "vertical" {
				self.run_yabai_command(&format!("-m window {} --toggle split", window.id));
			}
		}
	}

	pub fn move_window_to_master(&self, window: &Window) {
		log::debug!("Moving window {} to master", window.app);

		// Use a small heuristic that helps prevent "glitchy" window rearrangements
		self.run_yabai_command(&format!("-m window {} --warp east", window.id));

		// If the window is already a master window, then don't do anything
		if self.is_master_window(window) {
			return;
		}

		// Find a window that's touching the right side of the screen
		let master_window = some_or_return!(self.get_widest_master_window());

		if master_window.id == window.id {
			return;
		}

		self.run_yabai_command(&format!(
			"-m window {} --warp {}",
			window.id, master_window.id
		));

		let window = some_or_return!(self.get_updated_window_data(window));

		if window.split == "vertical" {
			self.run_yabai_command(&format!("-m window {} --toggle split", window.id));
		}
	}

	/**
	 * A window which is to the right of the dividing line is considered a master window.
	 */
	pub fn is_master_window(&self, window: &Window) -> bool {
		let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate();
		window.frame.x >= dividing_line_x_coordinate
	}

	pub fn is_window_touching_left_edge(&self, window: &Window) -> bool {
		window.frame.x == self.display.frame.x
	}

	/**
	 * If the window's frame has an x equal to the x of the display, it is a stack window
	 */
	pub fn is_stack_window(&self, window: &Window) -> bool {
		self.is_window_touching_left_edge(window)
	}

	pub fn is_middle_window(&self, window: &Window) -> bool {
		!self.is_stack_window(window) && !self.is_master_window(window)
	}

	pub fn get_middle_windows(&self) -> Vec<&Window> {
		self.windows_data
			.iter()
			.filter(|window| self.is_middle_window(window))
			.collect()
	}

	pub fn get_master_windows(&self) -> Vec<&Window> {
		let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate();
		self.windows_data
			.iter()
			.filter(|window| window.frame.x >= dividing_line_x_coordinate)
			.collect::<Vec<&Window>>()
	}

	pub fn get_stack_windows(&self) -> Vec<&Window> {
		self.windows_data
			.iter()
			.filter(|window| self.is_stack_window(window))
			.collect()
	}

	pub fn get_top_window<'w>(&self, windows: &'w Vec<&Window>) -> Option<&'w Window> {
		if windows.len() == 0 {
			return None;
		}

		let top_window = windows[0];
		for w in windows {
			if w.frame.y < top_window.frame.y {
				top_window = w;
			}
		}

		Some(top_window)
	}

	pub fn is_top_window(&self, windows: &Vec<&Window>, window: &Window) -> bool {
		self.get_top_window(windows)
			.and_then(|top_window| Some(top_window.id == window.id))
			.unwrap_or(false)
	}

	pub fn get_bottom_window<'w>(&self, windows: &'w Vec<&Window>) -> Option<&'w Window> {
		if windows.len() == 0 {
			return None;
		}

		let bottom_window = windows[0];
		for w in windows {
			if w.frame.y > bottom_window.frame.y {
				bottom_window = w;
			}
		}

		Some(bottom_window)
	}

	pub fn is_bottom_window(&self, windows: &Vec<&Window>, window: &Window) -> bool {
		self.get_bottom_window(windows)
			.and_then(|bottom_window| Some(bottom_window.id == window.id))
			.unwrap_or(false)
	}

	pub fn get_top_stack_window(&self) -> Option<&Window> {
		self.get_top_window(&self.get_stack_windows())
	}

	pub fn get_bottom_stack_window(&self) -> Option<&Window> {
		self.get_bottom_window(&self.get_stack_windows())
	}

	pub fn get_top_master_window(&self) -> Option<&Window> {
		self.get_top_window(&self.get_master_windows())
	}

	pub fn get_bottom_master_window(&self) -> Option<&Window> {
		self.get_bottom_window(&self.get_master_windows())
	}
}
