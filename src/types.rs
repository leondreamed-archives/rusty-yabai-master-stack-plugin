use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Frame {
	pub x: usize,
	pub y: usize,
	pub w: usize,
	pub h: usize,
}

#[derive(Deserialize)]
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
	pub opacity: usize,
	pub shadow: usize,
	pub border: usize,

	#[serde(rename = "kebab-case")]
	pub stackIndex: usize,

	#[serde(rename = "kebab-case")]
	pub zoomParent: usize,

	#[serde(rename = "kebab-case")]
	pub zoomFullscreen: usize,

	#[serde(rename = "kebab-case")]
	pub nativeFullscreen: usize,
}

#[derive(Deserialize)]
pub struct Display {
	pub id: usize,
	pub uuid: usize,
	pub index: usize,
	pub spaces: Vec<usize>,
	pub frame: Frame,
}

#[derive(Deserialize)]
pub struct Space {
	pub id: String,
	pub label: String,
	pub index: usize,
	pub display: usize,
	pub windows: Vec<usize>,
	pub r#type: String,
	pub visible: usize,
	pub focused: usize,

	#[serde(rename = "kebab-case")]
	pub nativeFullscreen: usize,

	#[serde(rename = "kebab-case")]
	pub firstWindow: usize,

	#[serde(rename = "kebab-case")]
	pub lastWindow: usize,
}