mod context;
mod types;
mod macros;

use load_dotenv::try_load_dotenv;

try_load_dotenv!();

fn main() {
	println!("Hello, world!");
}
