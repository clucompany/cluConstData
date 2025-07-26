///
/// The safe and recommended method of the description of constant data.
#[macro_export]
macro_rules! const_data {
	//&'static str
	[$(pub $(($p_tt:tt))*)* const $name: ident : & $l: lifetime str = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		$(pub $(($p_tt))*)* const $name: & $l str = $crate::concat_const_raw!(@as_bytes: $a, $($b),*);

		$crate::const_data! {$($tt)*}
	};

	//&'static [u8]
	[$(pub $(($p_tt:tt))*)* const $name: ident : & $l: lifetime [$type: ty] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		$(pub $(($p_tt))*)* const $name: & $l [$type] = &$crate::concat_const_raw!($type: $a, $($b),*);

		$crate::const_data! {$($tt)*}
	};

	//&'static [u8; usize]
	[$(pub $(($p_tt:tt))*)* const $name: ident : & $l: lifetime [$type: ty;$size:expr] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		$(pub $(($p_tt))*)* const $name: & $l [$type;$size] = &$crate::concat_const_raw!($type: $a, $($b),*);

		$crate::const_data! {$($tt)*}
	};

	//Please, the very end!
	//&'static u8
	[$(pub $(($p_tt:tt))*)* const $name: ident : & $l: lifetime $type: ty = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		$(pub $(($p_tt))*)* const $name: & $l $type = &$crate::concat_const_raw!($type: $a, $($b),*);

		$crate::const_data! {$($tt)*}
	};
	//

	//[u8; usize]
	[$(pub $(($p_tt:tt))*)* const $name: ident : [$type: ty; $size:expr] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		$(pub $(($p_tt))*)* const $name: [$type; $size] = $crate::concat_const_raw!($type: $a, $($b),*);

		$crate::const_data! {$($tt)*}
	};

	//[u8]
	[$(pub $(($p_tt:tt))*)* const $name: ident : [$type: ty] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		$(pub $(($p_tt))*)* const $name: [$type] = $crate::concat_const_raw!($type: $a, $($b),*);

		$crate::const_data! {$($tt)*}
	};



	//Please, the very end!
	//T
	[$(pub $(($p_tt:tt))*)* const $name: ident : $ty: ty = $a:expr;		$($tt:tt)*] => {
		$(pub $(($p_tt))*)* const $name: $ty = $a;

		$crate::const_data! {$($tt)*}
	};


	//END
	() => ()
}

#[test]
#[cfg(test)]
fn one_const_data() {
	const_data! {
		const A: &'static [u8]		= b"123";
		const B: &'static str		= "123";
		const C: u32			= 10;
	}

	assert_eq!(A, b"123");
	assert_eq!(B, "123");
	assert_eq!(C, 10);
}

#[test]
#[cfg(test)]
fn u8_array_const_data() {
	const_data! {
		const A: &'static [u8]		= b"123";
		const B: &'static [u8]		= b".end";

		const ARRAY: &'static [u8]	= A, B, b"1234";
		const ARRAY2: &'static [u8]	= ARRAY, b"1234", b".";
	}

	assert_eq!(A, b"123");
	assert_eq!(B, b".end");

	assert_eq!(ARRAY, b"123.end1234");
	assert_eq!(ARRAY2, b"123.end12341234.");
}

#[test]
#[cfg(test)]
fn str_array_const_data() {
	const_data! {
		const A: &'static str		= "123";
		const B: &'static str		= ".end";

		const ARRAY: &'static str	= A, B, "1234";
		const ARRAY2: &'static str	= ARRAY, "1234", ".";
	}

	assert_eq!(A, "123");
	assert_eq!(B, ".end");

	assert_eq!(ARRAY, "123.end1234");
	assert_eq!(ARRAY2, "123.end12341234.");
}
