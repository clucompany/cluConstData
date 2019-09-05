
#[macro_use]
extern crate cluConstData;

const_data! {
	const S_PREFIX:		&'static str	= "L[";
	const E_PREFIX:		&'static str 	= "]";
	
	const MY_STR:			&'static str	= S_PREFIX, "->", E_PREFIX;
}

fn main() {
	println!("SINGLE_DATA: {:?}", const_single_data!([u8; 2] = b"1", b"2"));
	assert_eq!(b"12", &const_single_data!([u8; 2] = b"1", b"2"));
	
	println!("CONST_STR: {:?}", const_single_data!(&'static str = "!", MY_STR, "!"));
	assert_eq!("!L[->]!", const_single_data!(&'static str = "!", MY_STR, "!"));
}
