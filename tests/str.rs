use cluConstData::concat_str;

#[test]
fn concat_str() {
	const H: &str = "H";
	const E: &str = "e";
	const L: &str = "l";
	const O: &str = "o";
	const RIGHT: &str = "!";

	const HELLO_WORLD: &str = concat_str!(H, E, L, L, O, " ", "World", RIGHT);

	assert!(core::str::from_utf8(HELLO_WORLD.as_bytes()).is_ok());
	assert_eq!(HELLO_WORLD, "Hello World!");
}
