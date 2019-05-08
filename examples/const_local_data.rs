
#[macro_use]
extern crate cluConstData;

const MY_DATA: &'static str = "[][][..#ULIN";

fn main() {
	const_data! {
		const LOCAL_DATA:		&'static str = MY_DATA, " ", "OK:)";
		const LOCAL_DATA2:	&'static str = LOCAL_DATA, "2";
	}
	println!("{:?}", LOCAL_DATA);
	assert_eq!(LOCAL_DATA, "[][][..#ULIN OK:)");
	assert_eq!(LOCAL_DATA.len(), "[][][..#ULIN OK:)".len());
	
	println!("{:?}", LOCAL_DATA2);
	assert_eq!(LOCAL_DATA2, "[][][..#ULIN OK:)2");
	assert_eq!(LOCAL_DATA2.len(), "[][][..#ULIN OK:)2".len());
}