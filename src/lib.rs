//Copyright 2019 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1819
//

/*!

Safe constant combination of constant data.

# Use

1. Easy

```
#[macro_use]
extern crate cluConstConcat;

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
	
	
	println!("E_PREFIX: {:?}", S_PREFIX);
	assert_eq!(E_PREFIX, "]");
	assert_eq!(E_PREFIX.len(), 1);
	assert_eq!(S_PREFIX.as_bytes(), b"]");
	
	println!("MY_STR: {:?}", MY_STR);
	assert_eq!(MY_STR, "L[->]");
	assert_eq!(MY_STR.len(), 5);
	assert_eq!(MY_STR.as_bytes(), b"L[->]");
	
	println!("TWO_MY_STR: {:?}", TWO_MY_STR);
	assert_eq!(TWO_MY_STR, "L[->]L[->]");
	assert_eq!(TWO_MY_STR.len(), 10);
	assert_eq!(MY_STR.as_bytes(), b"L[->]L[->]");
}
```

2. ArrayUse

```
#[macro_use]
extern crate cluConstConcat;

const_data! {
	const U32_HEAD:u32			= 255;
	const U32_END:u32			= 0;
	
	const U32_ARRAY:[u32; 3]		= &[U32_HEAD], &[2], &[U32_END];
	const U32_SARRAY:&'static [u32]	= &[U32_HEAD, 2, 3 ,4], &[2, 3], &[U32_END];
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
```

3. TraitUse

```
#[macro_use]
extern crate cluConstConcat;

use std::marker::PhantomData;

fn main() {
	println!("TypeTrait<usize>: {:?} \"{}\"", usize::RAW_TYPE, unsafe {std::str::from_utf8_unchecked(usize::RAW_TYPE)} );
	assert_eq!(usize::RAW_TYPE, b"usize");
	
	println!("TypeTrait<usize + usize>: {:?} \"{}\"", <(usize, usize)>::RAW_TYPE, unsafe {std::str::from_utf8_unchecked(<(usize, usize)>::RAW_TYPE)} );
	assert_eq!(<(usize, usize)>::RAW_TYPE, b"usize + usize");
}

pub trait TypeTrait {
	const TYPE: &'static str;
	const RAW_TYPE: &'static [u8];
}

impl TypeTrait for (usize, usize) {
	const_data! {
		const TYPE: &'static str = usize::TYPE, " + ", usize::TYPE;
		const RAW_TYPE: &'static [u8] = usize::RAW_TYPE, b" + ", usize::RAW_TYPE;
	}
}

impl TypeTrait for (PhantomData<()>, usize) {
	const_data! {
		const TYPE: &'static str = "PhantomData<()>", " + ", usize::TYPE;
		const RAW_TYPE: &'static [u8] = b"PhantomData<()>", b" ", usize::RAW_TYPE;
	}
}

impl TypeTrait for usize {
	const_data! {
		const TYPE: &'static str = "usize";
		const RAW_TYPE: &'static [u8] = b"usize";
	}
}

impl TypeTrait for u8 {
	const_data! {
		const TYPE: &'static str = "u8";
		const RAW_TYPE: &'static [u8] = b"u8";
	}
}

impl TypeTrait for u32 {
	const_data! {
		const TYPE: &'static str = "u32";
		const RAW_TYPE: &'static [u8] = b"u32";
	}
}

impl TypeTrait for u64 {
	const_data! {
		const TYPE: &'static str = "u64";
		const RAW_TYPE: &'static [u8] = b"u64";
	}
}
```

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
*/

#![feature(const_fn_union)]
#![feature(untagged_unions)]
#![feature(const_fn)]
#![feature(const_slice_len)]
#![feature(const_str_as_bytes)]
#![feature(const_raw_ptr_deref)]
#![feature(const_str_len)]

#[allow(unions_with_drop_fields)]
union UnionTransmute<Value, To> {
	value: Value,
	// &'static [u8]
	
	to: To,
	// &[u8; 1024]
}

impl<A, B> UnionTransmute<A, B> {
	#[inline(always)]
	pub const unsafe fn into(value: A) -> B {
		Self { value }.to
	}
}


#[repr(C)]
#[derive(Debug)]
pub struct ConstConcat<A, B> {
	a: A,
	b: B,
}

impl<DataLeft, DataRight> ConstConcat<DataLeft, DataRight> where DataLeft: Copy, DataRight: Copy {
	///Very coarse concatenation, use safe macros such as 'const_data' !!
	pub const unsafe fn const_concat<DataTo, T>(a: &[T], b: &[T]) -> DataTo {
		let result = Self {
			a: *UnionTransmute::<_, &DataLeft>::into(a),
			//Transmute
			//&[T] -> &DataLeft  (DataLeft: &[T; 1024])
			//
			//and copy data!
			//&[T; 1024] -> (a: New [T; 1024] )
			//
			
			b: *UnionTransmute::<_, &DataRight>::into(b),
		};
		//result: 
		//R<DataLeft, DataRight> (R<[T; 1024], [T; 1024]>)
		//
		
		UnionTransmute::into(result)
		//Transmute result.
		//
		//R<[T; 1024], [T; 1024]> -> [T; 1024 + 1024]
		//
	}	
	
}

///To use only together with our library.
pub mod ignore_feature {
	///Ignore #![feature(const_raw_ptr)]
	#[inline(always)]
	pub const unsafe fn const_raw_ptr(a: &[u8]) -> &str {
		&*(a as *const [u8] as *const str)	
	}
	
	///Ignore #![feature(const_str_as_bytes)]
	#[inline(always)]
	pub const unsafe fn const_str_as_bytes(a: &str) -> &[u8] {
		a.as_bytes()
	}
	
	///Ignore #![feature(const_slice_len)]
	#[inline(always)]
	pub const unsafe fn const_slice_len<T>(a: &[T]) -> usize {
		a.len()	
	}
	
	///Ignore #![feature(const_str_len)]
	#[inline(always)]
	pub const unsafe fn const_str_len(a: &str) -> usize {
		a.len()	
	}
}


#[inline(always)]
///Very coarse concatenation, use safe macros such as 'const_data' !!
pub const unsafe fn const_concat<'a, DataLeft, DataRight, DataTo, T>(a: &'a [T], b: &'a [T]) -> DataTo where DataLeft: Copy, DataRight: Copy {
	ConstConcat::<DataLeft, DataRight>::const_concat::<DataTo, T>(a, b)
}

///
///The safe and recommended method of the description of constant data.
///```
///#[macro_use]
///extern crate cluConstConcat;
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
///extern crate cluConstConcat;
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