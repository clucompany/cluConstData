
#[macro_use]
extern crate cluConstData;

const_data! {
	const S_PREFIX:			&'static str	= "L[";
	const E_PREFIX:			&'static str 	= "]";
	
	const MY_STR:			&'static str	= S_PREFIX, "->", E_PREFIX;
	const TWO_MY_STR:			&'static str	= MY_STR, MY_STR;
}

fn main() {
	println!("S_PREFIX: {:?}", S_PREFIX);
	assert_eq!(S_PREFIX, "L[");
	assert_eq!(S_PREFIX.len(), 2);
	assert_eq!(S_PREFIX.as_bytes(), b"L[");
	
	
	println!("E_PREFIX: {:?}", E_PREFIX);
	assert_eq!(E_PREFIX, "]");
	assert_eq!(E_PREFIX.len(), 1);
	assert_eq!(E_PREFIX.as_bytes(), b"]");
	
	println!("MY_STR: {:?}", MY_STR);
	assert_eq!(MY_STR, "L[->]");
	assert_eq!(MY_STR.len(), 5);
	assert_eq!(MY_STR.as_bytes(), b"L[->]");
	
	println!("TWO_MY_STR: {:?}", TWO_MY_STR);
	assert_eq!(TWO_MY_STR, "L[->]L[->]");
	assert_eq!(TWO_MY_STR.len(), 10);
	assert_eq!(TWO_MY_STR.as_bytes(), b"L[->]L[->]");
}