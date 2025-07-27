use cluConstData::{concat_bytes, concat_str};

const A: &str = "const_";
const B: &str = "data";
const FULL: &str = concat_str!(A, B);

fn main() {
	println!("Merged string: {}", FULL);
	let debug: &[u8] = concat_bytes!(A.as_bytes(), B.as_bytes());
	println!("Debug as bytes: {:?}", debug);
}
