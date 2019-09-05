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
	
	
	println!("E_PREFIX: {:?}", S_PREFIX);
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
```

2. ArrayUse

```
#[macro_use]
extern crate cluConstData;

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
extern crate cluConstData;

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

4. SingleUse

```
#[macro_use]
extern crate cluConstData;

const_data! {
	const S_PREFIX:			&'static str	= "L[";
	const E_PREFIX:			&'static str 	= "]";
	
	const MY_STR:			&'static str	= S_PREFIX, "->", E_PREFIX;
}

fn main() {
	println!("SINGLE_DATA: {:?}", const_single_data!([u8; 2] = b"1", b"2"));
	assert_eq!(b"12", &const_single_data!([u8; 2] = b"1", b"2"));
	
	println!("CONST_STR: {:?}", const_single_data!(&'static str = "!", MY_STR, "!"));
	assert_eq!("!L[->]!", const_single_data!(&'static str = "!", MY_STR, "!"));
}
```

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
*/

#![allow(non_snake_case)]

#![feature(const_fn)]
#![feature(const_slice_len)]
#![feature(const_str_as_bytes)]
#![feature(const_raw_ptr_deref)]
#![feature(const_str_len)]

#[macro_use]
mod macros;
pub use self::macros::*;

#[macro_use]
mod macros_single_data;
pub use self::macros_single_data::*;

#[macro_use]
pub mod core {
	#[macro_use]
	mod concat;
	pub use self::concat::*;	
}

use cluFullTransmute::mem::full_transmute;

#[repr(C)]
#[derive(Debug)]
pub struct ConstConcat<A, B> {
	a: A,
	b: B,
}

impl<A, B> ConstConcat<A, B> where A: Copy, B: Copy {
	/// Very coarse concatenation, use safe macros such as 'const_data' !!
	pub const unsafe fn auto_const_concat<'a, DataTo, T>(a: &'a [T], b: &'a [T]) -> DataTo {
		let result = Self {
			a: *full_transmute::<_, &A>(a),
			// Transmute
			// &[T] -> &DataLeft  (DataLeft: &[T; 1024])
			//
			// and copy data!
			// &[T; 1024] -> (a: New [T; 1024] )
			//
			
			b: *full_transmute::<_, &B>(b),
		};
		// result: 
		// R<DataLeft, DataRight> (R<[T; 1024], [T; 1024]>)
		//
		
		full_transmute(result)
		// Transmute result.
		//
		// R<[T; 1024], [T; 1024]> -> [T; 1024 + 1024]
		//
	}	
}

/// Internal methods required by the library.
#[doc(hidden)]
pub mod ignore_feature {
	/// Ignore #![feature(const_raw_ptr)]
	#[inline(always)]
	pub const unsafe fn const_raw_ptr(a: &[u8]) -> &str {
		&*(a as *const [u8] as *const str)
	}
	
	/// Ignore #![feature(const_str_as_bytes)]
	#[inline(always)]
	pub const unsafe fn const_str_as_bytes(a: &str) -> &[u8] {
		a.as_bytes()
	}
	
	/// Ignore #![feature(const_slice_len)]
	#[inline(always)]
	pub const unsafe fn const_slice_len<T>(a: &[T]) -> usize {
		a.len()
	}
	
	/// Ignore #![feature(const_str_len)]
	#[inline(always)]
	pub const unsafe fn const_str_len(a: &str) -> usize {
		a.len()
	}
}


#[inline(always)]
/// Very coarse concatenation, use safe macros such as 'const_data' !!
pub const unsafe fn const_concat<'a, A, B, T, DataTo>(a: &'a [T], b: &'a [T]) -> DataTo where A: Copy, B: Copy {
	ConstConcat::<A, B>::auto_const_concat::<DataTo, T>(a, b)
}


/// Raw concatenation, see the description of the macro!
#[doc(hidden)]
#[macro_export]
macro_rules! raw_one_const {
	[$type:ty: $a: expr] => {$a};
	
	[str: $a: expr, $b: expr] => {{
		unsafe {
			$crate::ignore_feature::const_raw_ptr(
				&$crate::raw_one_const!{
					u8:
						unsafe { $crate::ignore_feature::const_str_as_bytes($a) }, 
						unsafe { $crate::ignore_feature::const_str_as_bytes($b) }
				}
			)
		}
	}};
	
	[str: $a: expr, $($b: expr),*] => {{
		//const _NO_VISIBLE: &'static str = $crate::raw_one_const!(str[$($p_tt)*]: $($b),*);
		
		//$crate::raw_one_const!(str[$($p_tt)*]: $a, _NO_VISIBLE)
		$crate::raw_one_const!(str: $a, $crate::raw_one_const!(str: $($b),*))
	}};
	
	[$type:ty: $a: expr, $b: expr] => {{
		#[allow(unused_unsafe)]
		unsafe {
			//#[allow(const_err)]
			//const __A_SIZE: usize = unsafe { $crate::ignore_feature::const_slice_len($a) };
			//#[allow(const_err)]
			//const __B_SIZE: usize = unsafe { $crate::ignore_feature::const_slice_len($b) };
			
			//let __A_SIZE: usize = unsafe { $crate::ignore_feature::const_slice_len($a) };
			//let __B_SIZE: usize = unsafe { $crate::ignore_feature::const_slice_len($b) };
			
			$crate::const_concat::<
				[$type; unsafe { $crate::ignore_feature::const_slice_len($a) }], 
				[$type; unsafe { $crate::ignore_feature::const_slice_len($b) }],
				$type,
				
				[$type; 
					unsafe { $crate::ignore_feature::const_slice_len($a) } + 
					unsafe { $crate::ignore_feature::const_slice_len($b) }
				],
			>($a, $b)
		}
	}};
	
	[$type:ty: $a: expr, $($b: expr),*] => {{
		//const _NO_VISIBLE: &'static [$type] = &$crate::raw_one_const!($type[$($p_tt:tt)*]: $($b),*);
		
		//$crate::raw_one_const!($type[$($p_tt)*]: $a, _NO_VISIBLE)
		$crate::raw_one_const!($type: $a, &$crate::raw_one_const!($type: $($b),*))
	}};
	
	
}

#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	
	#[test]
	fn generic_test() {
		trait AGeneric {
			const STR: &'static str;
			
			#[inline]
			fn as_str() -> &'static str {
				Self::STR
			}
		}
		struct A;
		struct B;
		
		impl AGeneric for A {
			const STR: &'static str = "A";
		}
		impl AGeneric for B {
			const STR: &'static str = "B";
		}
		
		impl AGeneric for (A, B) {
			const_data! {
				const STR: &'static str = A::STR, " + ", B::STR;
			}
		}
		
		assert_eq!(<(A, B)>::as_str(), "A + B");
	}
	

	/*#[test]
	fn full_generic_test() {
		pub trait ADyn {
			const STR: &'static str;
			
			#[inline]
			fn as_str() -> &'static str {
				Self::STR
			}
			
			#[inline]
			fn as_self_str() -> &'static str {
				&*Self::STR
			}
		}
		
		/*struct A;
		struct B;
		impl ADyn for A {
			const STR_DATA: &'static str = "1";
		}
		
		impl ADyn for B {
			const STR_DATA: &'static str = "2";
		}*/
		
		
		impl<T, T2> ADyn for (T, T2) where T: ADyn, T2: ADyn {
			const_data! {
				const STR: &'static str = <T as ADyn>::STR, " ", <T2 as ADyn>::STR;
			}
		}
		/*
		the trait bound `T: tests::full_generic_test::ADyn` is not satisfied

		the trait `tests::full_generic_test::ADyn` is not implemented for `T`

		help: consider adding a `where T: tests::full_generic_test::ADyn` boundrustc(E0277)
		lib.rs(374, 4): required by `tests::full_generic_test::ADyn::STR`
		lib.rs(400, 31): the trait `tests::full_generic_test::ADyn` is not implemented for `T`
		
		
		// I can not implement the full set of APIs. 
		// There is a restriction in the raster; 
		// it cannot trace the generic used for type T in constant types.
		
		// https://github.com/rust-lang/rust/issues/64077
		
		// Maybe someone will find a way?
		
		:(
		*/
	}*/
}