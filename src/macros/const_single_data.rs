
/// Safe constructor of constants and uniques without binding to data name..
#[macro_export]
macro_rules! const_single_data {
	//
	//&'lifetime str
	[& $l: lifetime str = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l str = $crate::raw_one_const!(str: $a $(, $b)*);
		_HIDDEN
		//let data: & $l str = $crate::raw_one_const!(str[& $l str]: $a $(, $b)*);
		//data
		
		//$crate::raw_one_const!(str: $a, $($b),*)
	}};
	
	//&'lifetime [u8]
	[& $l: lifetime [$type: ty] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l [$type] = &$crate::raw_one_const!($type: $a $(, $b)*);
		_HIDDEN
		//let data: & $l [$type] = &$crate::raw_one_const!($type[& $l [$type]]: $a $(, $b)*);
		//data
		
		//&$crate::raw_one_const!($type: $a, $($b),*)
	}};
	
	//&'lifetime [u8]
	[& $l: lifetime [$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l [$type; $size] = &$crate::raw_one_const!($type: $a $(, $b)*);
		_HIDDEN
		//let data: & $l [$type; $size] = &$crate::raw_one_const!($type[& $l [$type;$size]]: $a $(, $b)*);
		//data
		
		//&$crate::raw_one_const!($type: $a, $($b),*)
	}};
	//

	//[u8; 10]
	[[$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: [$type; $size] = $crate::raw_one_const!($type: $a $(, $b)*);
		_HIDDEN
		//let data: [$type; $size] = $crate::raw_one_const!($type[[$type;$size]]: $a $(, $b)*);
		//data
		
		//$crate::raw_one_const!($type: $a, $($b),*)
	}};
	
	//[u8]
	[[$type: ty] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: [$type] = $crate::raw_one_const!($type: $a $(, $b)*);
		_HIDDEN
		//let data: [$type] = $crate::raw_one_const!($type[[$type]]: $a $(, $b)*);
		//data
		
		//$crate::raw_one_const!($type: $a, $($b),*)
	}};
	
	//u8
	[$type: ty = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: $type = $crate::raw_one_const!($type: $a $(, $b)*);
		_HIDDEN
		//let data: $type = $crate::raw_one_const!($type[$type]: $a $(, $b)*);
		//data
		
		//$crate::raw_one_const!($type: $a, $($b),*)
	}};
	
	//EXPR
	[$ty: ty = $a:expr] => {{
		const _HIDDEN: $ty = $a;
		_HIDDEN
		//let data: $ty = $a;
		//data
		
		$a
	}};
	
	() => ()
}

/// Safe constructor of constants and uniques without binding to data name..
#[macro_export]
macro_rules! let_single_data {
	//
	//&'lifetime str
	[& $l: lifetime str = $a:expr, $($b:expr),*] => {{
		let data: & $l str = $crate::raw_one_const!(str: $a $(, $b)*);
		data
	}};
	
	//&'lifetime [u8]
	[& $l: lifetime [$type: ty] = $a:expr, $($b:expr),*] => {{
		let data: & $l [$type] = &$crate::raw_one_const!($type: $a $(, $b)*);
		data
	}};
	
	//&'lifetime [u8]
	[& $l: lifetime [$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		let data: & $l [$type; $size] = &$crate::raw_one_const!($type: $a $(, $b)*);
		data
	}};
	//

	//[u8; 10]
	[[$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		let data: [$type; $size] = $crate::raw_one_const!($type: $a $(, $b)*);
		data
	}};
	
	//[u8]
	[[$type: ty] = $a:expr, $($b:expr),*] => {{
		let data: [$type] = $crate::raw_one_const!($type: $a $(, $b)*);
		data
	}};
	
	//u8
	[$type: ty = $a:expr, $($b:expr),*] => {{
		let data: $type = $crate::raw_one_const!($type: $a $(, $b)*);
		data
	}};
	
	//EXPR
	[$ty: ty = $a:expr] => {{
		let data: $ty = $a;
		data
	}};
	
	() => ()
}

#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	
	
	#[test]
	fn two_single_data() {
		assert_eq!("!!", const_single_data!(&'static str = "!", "!"));
	}
	
	#[test]
	fn full_single_data() {
		const_data! {
			const A_PREFIX:			&'static str	= "[";
			const C_PREFIX:			&'static str 	= "]";
			
			const DATA:				&'static str	= A_PREFIX, "->", C_PREFIX;
		}

		assert_eq!(b"12", &const_single_data!([u8; 2] = b"1", b"2"));
		assert_eq!("![->]!", const_single_data!(&'static str = "!", DATA, "!"));
	}
	
}