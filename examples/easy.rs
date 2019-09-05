
#[macro_use]
extern crate cluConstData;

const_data! {
	const A: &'static [u8]	  = b"[[";
	const B: &'static [u8]	  = b"]]x";
	
	pub (crate) const ARRAY: &'static [u8] = A, B, b".end";
}

fn main() {
	assert_eq!(A, b"[[");
	assert_eq!(B, b"]]x");
	
	assert_eq!(ARRAY, b"[[]]x.end");
}