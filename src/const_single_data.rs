/// Safe constructor of constants and uniques without binding to data name..
#[macro_export]
macro_rules! const_single_data {
	//
	//&'lifetime str
	[& $l: lifetime str = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l str = $crate::concat_str!($a $(, $b)*);
		_HIDDEN
		//let data: & $l str = $crate::concat_const_raw!(str[& $l str]: $a $(, $b)*);
		//data

		//$crate::concat_const_raw!(@as_bytes: $a, $($b),*)
	}};

	//&'lifetime [u8]
	[& $l: lifetime [$type: ty] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l [$type] = &$crate::concat_bytes!(:[$type] = $a $(, $b)*);
		_HIDDEN
		//let data: & $l [$type] = &$crate::concat_const_raw!($type[& $l [$type]]: $a $(, $b)*);
		//data

		//&$crate::concat_const_raw!($type: $a, $($b),*)
	}};

	//&'lifetime [u8]
	[& $l: lifetime [$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l [$type; $size] = &$crate::concat_bytes!(:[$type] = $a $(, $b)*);
		_HIDDEN
		//let data: & $l [$type; $size] = &$crate::concat_const_raw!($type[& $l [$type;$size]]: $a $(, $b)*);
		//data

		//&$crate::concat_const_raw!($type: $a, $($b),*)
	}};
	//

	//[u8; 10]
	[[$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: [$type; $size] = $crate::concat_bytes!(:[$type] = $a $(, $b)*);
		_HIDDEN
		//let data: [$type; $size] = $crate::concat_const_raw!($type[[$type;$size]]: $a $(, $b)*);
		//data

		//$crate::concat_const_raw!($type: $a, $($b),*)
	}};

	//[u8]
	[[$type: ty] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: [$type] = $crate::concat_bytes!(:[$type] = $a $(, $b)*);
		_HIDDEN
		//let data: [$type] = $crate::concat_const_raw!($type[[$type]]: $a $(, $b)*);
		//data

		//$crate::concat_const_raw!($type: $a, $($b),*)
	}};

	//u8
	[$type: ty = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: $type = $crate::concat_bytes!(:[$type] = $a $(, $b)*);
		_HIDDEN
		//let data: $type = $crate::concat_const_raw!($type[$type]: $a $(, $b)*);
		//data

		//$crate::concat_const_raw!($type: $a, $($b),*)
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
		let data: & $l str = $crate::concat_str!($a $(, $b)*);
		data
	}};

	//&'lifetime [u8]
	[& $l: lifetime [$type: ty] = $a:expr, $($b:expr),*] => {{
		let data: & $l [$type] = &$crate::concat_bytes!([$type]: $a $(, $b)*);
		data
	}};

	//&'lifetime [u8]
	[& $l: lifetime [$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		let data: & $l [$type; $size] = &$crate::concat_bytes!([$type]: $a $(, $b)*);
		data
	}};
	//

	//[u8; 10]
	[[$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		let data: [$type; $size] = $crate::concat_bytes!([$type]: $a $(, $b)*);
		data
	}};

	//[u8]
	[[$type: ty] = $a:expr, $($b:expr),*] => {{
		let data: [$type] = $crate::concat_bytes!([$type]: $a $(, $b)*);
		data
	}};

	//u8
	[$type: ty = $a:expr, $($b:expr),*] => {{
		let data: $type = $crate::concat_bytes!([$type]: $a $(, $b)*);
		data
	}};

	//EXPR
	[$ty: ty = $a:expr] => {{
		let data: $ty = $a;
		data
	}};

	() => ()
}

#[test]
#[cfg(test)]
fn two_single_data() {
	assert_eq!("!!", const_single_data!(&'static str = "!", "!"));
}

#[test]
#[cfg(test)]
fn full_single_data() {
	crate::const_data! {
		const A_PREFIX:			&'static str	= "[";
		const C_PREFIX:			&'static str 	= "]";

		const DATA:				&'static str	= A_PREFIX, "->", C_PREFIX;
	}

	assert_eq!(b"12", &const_single_data!([u8; 2] = b"1", b"2"));
	assert_eq!("![->]!", const_single_data!(&'static str = "!", DATA, "!"));
}
