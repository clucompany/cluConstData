
#[macro_use]
extern crate cluConstConcat;

const_data! {
	const U32_HEAD:			u32			= 255;
	const U32_END:			u32			= 0;
	
	const U32_ARRAY:			[u32; 3]		= &[U32_HEAD], &[2], &[U32_END];
	const U32_SARRAY:			&'static [u32]	= &[U32_HEAD, 2, 3 ,4], &[2, 3], &[U32_END];
}

fn main() {
	println!("U32_HEAD: {:?}", U32_HEAD);
	assert_eq!(U32_HEAD, 255);
	
	println!("U32_END: {:?}", U32_END);
	assert_eq!(U32_END, 0);
	
	println!("U32_ARRAY: {:?}", U32_ARRAY);
	assert_eq!(U32_ARRAY, [255, 2, 0]);
	
	println!("U32_SARRAY: {:?}", U32_SARRAY);
	assert_eq!(U32_SARRAY, [255, 2, 3, 4, 2, 3, 0]);
}