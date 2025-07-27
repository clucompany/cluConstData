use cluConstData::const_data;

const_data! {
	pub const HELLO: &str = "Hello";
	pub const GREETING: &str = HELLO, ", world!";
}

fn main() {
	println!("{GREETING}");
}
