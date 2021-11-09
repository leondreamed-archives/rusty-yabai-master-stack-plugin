use serde::{Deserialize};

#[derive(Clone, Deserialize)]
pub struct Frame {
	pub x: f64,
	pub y: f64,
	pub w: f64,
	pub h: f64,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Window {
	pub id: usize,
	pub pid: usize,
	pub app: String,
	pub title: String,
	pub frame: Frame,
	pub level: usize,
	pub role: String,
	pub subrole: String,
	pub movable: usize,
	pub resizable: usize,
	pub display: usize,
	pub space: usize,
	pub focused: usize,
	pub split: String,
	pub floating: usize,
	pub sticky: usize,
	pub minimized: usize,
	pub topmost: usize,
	pub opacity: f64,
	pub shadow: usize,
	pub border: usize,
	pub stack_index: usize,
	pub zoom_parent: usize,
	pub zoom_fullscreen: usize,
	pub native_fullscreen: usize,
}

#[derive(Clone, Deserialize)]
pub struct Display {
	pub id: usize,
	pub uuid: String,
	pub index: usize,
	pub spaces: Vec<usize>,
	pub frame: Frame,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Space {
	pub id: usize,
	pub label: String,
	pub index: usize,
	pub display: usize,
	pub windows: Vec<usize>,
	pub r#type: String,
	pub visible: usize,
	pub focused: usize,
	pub native_fullscreen: usize,
	pub first_window: usize,
	pub last_window: usize,
}