
///
///The safe and recommended method of the description of constant data.
///```
///#[macro_use]
///extern crate cluConstData;
///
///const_data! {
///	pub const L_PREFIX:	&'static [u8] = b"<";
///	pub const R_PREFIX:	&'static [u8] = b">";
///	
///	const MY_DATA:		&'static [u8] = L_PREFIX, b"Test", R_PREFIX;
///	const TEST:			[u8; 2] = L_PREFIX, R_PREFIX;
///}
///
///fn main() {
///	println!("L_PREFIX: {:?} \"{}\"", L_PREFIX, unsafe {std::str::from_utf8_unchecked(L_PREFIX)} );
///	assert_eq!(L_PREFIX, b"<");
///	
///	println!("R_PREFIX: {:?} \"{}\"", R_PREFIX, unsafe {std::str::from_utf8_unchecked(R_PREFIX)} );
///	assert_eq!(R_PREFIX, b">");
///	
///	println!("MY_DATA: {:?} \"{}\"", MY_DATA, unsafe {std::str::from_utf8_unchecked(MY_DATA)} );
///	assert_eq!(MY_DATA, b"<Test>");
///	
///	println!("TEST: {:?} \"{}\"", TEST, unsafe {std::str::from_utf8_unchecked(&TEST)} );
///	assert_eq!(&TEST, b"<>");
///}
///```

#[macro_export]
macro_rules! const_data {
	//CONCAT!
	[pub const $name: ident : & $l: lifetime str = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		pub const $name: & $l str = $crate::const_concat!(->&str: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	[const $name: ident : & $l: lifetime str = $a:expr, $($b:expr),*;		$($tt:tt)*] => {
		const $name: & $l str = $crate::const_concat!(->&str: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	
	
	//SLICE
	[pub const $name: ident : & $l: lifetime [$type: ty] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		pub const $name: & $l [$type] = &$crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	[const $name: ident : & $l: lifetime [$type: ty] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		const $name: & $l [$type] = &$crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	//SLICE;expr
	[pub const $name: ident : & $l: lifetime [$type: ty;$size:expr] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		pub const $name: & $l [$type;$size] = &$crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	[const $name: ident : & $l: lifetime [$type: ty;$size:expr] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		const $name: & $l [$type;$size] = &$crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	
	//
	
	//ARRAY
	[pub const $name: ident : [$type: ty; $size:expr] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		pub const $name: [$type; $size] = $crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	[const $name: ident : [$type: ty; $size:expr] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		const $name: [$type; $size] = $crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	//ARRAY One
	[pub const $name: ident : [$type: ty] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		pub const $name: [$type] = $crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	[const $name: ident : [$type: ty] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		const $name: [$type] = $crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	
	//EMPTY
	[pub const $name: ident : $ty: ty = $a:expr;		$($tt:tt)*] => {
		pub const $name: $ty = $a;
		
		$crate::const_data! {$($tt)*}
	};
	[const $name: ident : $ty: ty = $a:expr;			$($tt:tt)*] => {
		const $name: $ty = $a;
		
		$crate::const_data! {$($tt)*}
	};
	
	
	
	
	//END
	() => ()
}

///Raw concatenation, see the description of the macro!
#[macro_export]
macro_rules! const_concat {
	[->&str: $a: expr, $b: expr] => {
		unsafe {
			$crate::ignore_feature::const_raw_ptr(
				&$crate::ConstConcat::<
					[u8; unsafe{ $crate::ignore_feature::const_str_len($a) }], 
					[u8; unsafe{ $crate::ignore_feature::const_str_len($b) }]
				>::const_concat::<
					[u8; 
						unsafe{ $crate::ignore_feature::const_str_len($a) } + 
						unsafe{ $crate::ignore_feature::const_str_len($b) }
					], u8
				>(
					$crate::ignore_feature::const_str_as_bytes($a), 
					$crate::ignore_feature::const_str_as_bytes($b),
				)
			)	
		}
	};
	
	[->&str: $a: expr, $($b: expr),*] => {{
		const _NO_VISIBLE: &'static str = $crate::const_concat!(->&str: $($b),*);
		
		$crate::const_concat!(->&str: $a, _NO_VISIBLE)
	}};
	
	[$type: ty: $a: expr, $b: expr] => {
		unsafe {
			$crate::ConstConcat::<
				[$type; unsafe{ $crate::ignore_feature::const_slice_len($a) }], 
				[$type; unsafe{ $crate::ignore_feature::const_slice_len($b) }]
			>::const_concat::<
				[$type; 
					unsafe{ $crate::ignore_feature::const_slice_len($a) } + 
					unsafe{ $crate::ignore_feature::const_slice_len($b) }
				]
				, $type
			>
			
			($a, $b) 
		}
	};
	
	
	
	
	[@$type: ty : $a: expr, $($b: expr),*] => {{
		const _NO_VISIBLE: [$type] = $crate::const_concat!($type: $($b),*);
		
		$crate::const_concat!(@$type: $a, _NO_VISIBLE)
	}};
	
	[$type: ty : $a: expr, $($b: expr),*] => {{
		const _NO_VISIBLE: &'static [$type] = &$crate::const_concat!($type: $($b),*);
		
		$crate::const_concat!($type: $a, _NO_VISIBLE)
	}};
	
	
	
	[$a: expr, $($b: expr),*] => {
		$crate::const_concat!(u8: $a, $($b),*)	
	};
	
	[$type:ty: $a: expr] => {$a};
	//Ignore..
	[$a: expr] => {$a};
}

///Safe designer of single data.
///```
///#[macro_use]
///extern crate cluConstData;
///
///const_data! {
///	const S_PREFIX:			&'static str	= "L[";
///	const E_PREFIX:			&'static str 	= "]";
///	
///	const MY_STR:			&'static str	= S_PREFIX, "->", E_PREFIX;
///}
///
///fn main() {
///	println!("SINGLE_DATA: {:?}", const_single_data!([u8; 2] = b"1", b"2"));
///	assert_eq!(b"12", &const_single_data!([u8; 2] = b"1", b"2"));
///	
///	println!("CONST_STR: {:?}", const_single_data!(&'static str = "!", MY_STR, "!"));
///	assert_eq!("!L[->]!", const_single_data!(&'static str = "!", MY_STR, "!"));
///}
///```
#[macro_export]
macro_rules! const_single_data {
	[& $l: lifetime str = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l str = $crate::const_concat!(->&str: $a $(, $b)*);
		_HIDDEN
	}};
	
	//SLICE
	[& $l: lifetime [$type: ty] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l [$type] = &$crate::const_concat!($type: $a $(, $b)*);
		_HIDDEN
	}};
	[& $l: lifetime [$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: & $l [$type; $size] = &$crate::const_concat!($type: $a $(, $b)*);
		_HIDDEN
	}};

	//ARRAY
	[[$type: ty; $size:expr] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: [$type; $size] = $crate::const_concat!($type: $a $(, $b)*);
		_HIDDEN
	}};
	[[$type: ty] = $a:expr, $($b:expr),*] => {{
		const _HIDDEN: [$type] = $crate::const_concat!($type: $a $(, $b)*);
		_HIDDEN
	}};
	

	[$ty: ty = $a:expr] => {{
		const _HIDDEN: $ty = $a;
		_HIDDEN
	}};
	
	() => ()
}
