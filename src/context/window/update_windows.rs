use crate::context::window::{CheckValidLayoutPayload, CheckValidLayoutProps};

use super::WindowsManager;

pub struct UpdateWindowsProps {
	pub target_num_master_windows: usize,
}

impl WindowsManager<'_> {
	pub fn update_windows(
		&mut self,
		UpdateWindowsProps {
			target_num_master_windows,
		}: UpdateWindowsProps,
	) {
		log::debug!(
			"updateWindows() called with targetnum_master_windows = {}",
			target_num_master_windows
		);
		let layout_validity = self.check_valid_layout(CheckValidLayoutProps {
			target_num_master_windows: Some(target_num_master_windows),
		});

		if let CheckValidLayoutPayload::Failure(reason) = layout_validity {
			log::debug!("Invalid layout detected: {}. Updating windows...", reason);
		} else {
			log::debug!("Valid layout detected; no changes were made.");
			return;
		}

		let num_windows = self.windows_data.len();

		// If the stack is supposed to exist but doesn't exist
		if target_num_master_windows != num_windows && !self.does_stack_exist() {
			log::debug!("Stack does not exist, creating it...");
			self.create_stack();
		}

		if num_windows > 2 {
			let mut master_windows = self.get_master_windows();
			log::debug!(
				"Master windows: {:?}",
				master_windows
					.iter()
					.map(|w| &w.app)
					.collect::<Vec<&String>>()
			);

			let mut cur_num_master_windows = master_windows.len();

			// If there are too many master windows, move them to stack
			if cur_num_master_windows > target_num_master_windows {
				log::debug!(
					"Too many master windows ({}/{})",
					cur_num_master_windows,
					target_num_master_windows
				);
				// Sort the windows from bottom to top and then right to left
				master_windows.sort_by(|window1, window2| {
					if window1.frame.y != window2.frame.y {
						window1.frame.y.cmp(&window2.frame.y)
					} else {
						window1.frame.x.cmp(&window2.frame.x)
					}
				});

				while cur_num_master_windows > target_num_master_windows {
					// Remove the window with the greatest y-coordinate first
					if let Some(master_window) = master_windows.pop() {
						log::debug!("Moving master window {} to stack", master_window.app);
						self.move_window_to_stack(&master_window);
					}
					cur_num_master_windows -= 1;
				}
			}

			// If there are windows that aren't touching either the left side or the right side
			// after the move, fill up master and then move the rest to stack
			let middle_windows = self.get_middle_windows();
			while middle_windows.len() > 0 {
				let middle_window = &middle_windows[0];
				log::debug!("Middle window {} detected.", middle_window.app);
				if cur_num_master_windows < target_num_master_windows {
					log::debug!("Moving middle window {} to master.", middle_window.app);
					self.move_window_to_master(middle_window);
					cur_num_master_windows += 1;
				} else {
					log::debug!("Moving middle window {} to stack.", middle_window.app);
					self.move_window_to_stack(middle_window);
				}
			}

			// If there are still not enough master windows, move some of the stack windows to master
			let mut stack_windows = self.get_stack_windows();

			// Sort the stack windows by reverse y-coordinate and reverse x-coordinate to move the
			// bottom-rightmost windows first
			stack_windows.sort_by(|window1, window2| {
				if window1.frame.x != window2.frame.x {
					window2.frame.x.cmp(&window1.frame.x)
				} else {
					window2.frame.y.cmp(&window1.frame.y)
				}
			});

			while cur_num_master_windows < target_num_master_windows {
				log::debug!(
					"Not enough master windows ({}/{})",
					cur_num_master_windows,
					target_num_master_windows
				);
				if let Some(stack_window) = stack_windows.pop() {
					log::debug!("Moving stack window {} to master.", stack_window.app);
					self.move_window_to_master(&stack_window);
				}
				cur_num_master_windows += 1;
			}
		}

		// Note: the following should never be called
		if let CheckValidLayoutPayload::Failure(reason) =
			self.check_valid_layout(CheckValidLayoutProps {
				target_num_master_windows: Some(target_num_master_windows),
			}) {
			panic!(
				"updateLayout() ended with an invalid layuot; reason: {}",
				reason
			)
		} else {
			log::debug!("updateLayout() was successful.");
		}

		self.expected_current_num_master_windows = target_num_master_windows;
	}
}
