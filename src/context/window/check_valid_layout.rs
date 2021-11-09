use super::WindowsManager;

pub struct CheckValidLayoutProps {
	target_num_master_windows: Option<usize>,
}

pub enum CheckValidLayoutPayload {
	Success,
	Failure(String),
}

impl WindowsManager<'_> {
	pub fn check_valid_layout(&self, props: CheckValidLayoutProps) -> CheckValidLayoutPayload {
		log::debug!("Starting valid layout check...");

		// If there are no windows, it is a valid layout
		if self.windows_data.len() == 0 {
			return CheckValidLayoutPayload::Success;
		}

		let target_num_master_windows = props
			.target_num_master_windows
			.unwrap_or(self.expected_current_num_master_windows);

		// If targetNumMasterWindows is greater or equal to the number of windows, all windows must be touching the left side
		if target_num_master_windows >= self.windows_data.len()
			&& !self
				.windows_data
				.iter()
				.all(|w| self.is_window_touching_left_edge(&w))
		{
			return CheckValidLayoutPayload::Failure("The number of master windows is greater or equal to the number of windows and not all windows are touching the left edge.".to_string());
		} else {
			// Verify that the number of master windows equals the target number of master windows
			let cur_num_master_windows = self.get_master_windows().len();

			if target_num_master_windows != cur_num_master_windows {
				return CheckValidLayoutPayload::Failure(format!("Number of master windows does not equal expected number of master windows ({}/{})", cur_num_master_windows, target_num_master_windows));
			}

			// Verify that there is no middle window
			for window in self.windows_data {
				if self.is_middle_window(&window) {
					return CheckValidLayoutPayload::Failure(format!(
						"A middle window ({}) was detected.",
						window.app
					));
				}
			}

			return CheckValidLayoutPayload::Success;
		}
	}
}
