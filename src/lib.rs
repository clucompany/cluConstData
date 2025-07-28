//Copyright 2019-2025 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 2025
//

/*!

Create and merge any persistent data at compile time. A nightly compiler version is required (this is not a compiler plugin).

# Opportunities

1. Combining any persistent arrays at compile time
2. Combining any static strings at compile time
3. Ability to combine generic constant data (but only with known types (with unknown types Rust cannot track generic relationships)).
4. The library uses #! [no _ std]



# Use

1. Easy

```rust
#[macro_use]
extern crate cluConstData;

const_data! {
	const A: &'static [u8]	  = b"[";
	const B: &'static [u8]	  = b"].";

	pub (crate) const ARRAY: &'static [u8] = A, b"User", B, b" ";
}

fn main() {
	assert_eq!(A, b"[");
	assert_eq!(B, b"].");

	println!("#1 {}", std::str::from_utf8(ARRAY).unwrap());
	assert_eq!(ARRAY, b"[User]. ");
}
```

2. EasyStr

```rust
#[macro_use]
extern crate cluConstData;

const_data! {
	const A: &'static str	  = "[";
	const B: &'static str	  = "]";

	pub (crate) const RESULT: &'static str = A, "DATA", B;
}

fn main() {
	assert_eq!(A, "[");
	assert_eq!(B, "]");

	println!("#1 {}", RESULT);
	assert_eq!(RESULT, "[DATA]");
}
```


3. EasyArray

```rust
#[macro_use]
extern crate cluConstData;


const_data! {
	const U32_HEAD:	u32			= 255;
	const U32_END:		u32		= 0;


	const U32_ARRAY:	[u32; 3]		= &[U32_HEAD], &[2], &[U32_END];
	const U32_SARRAY:	&'static [u32]	= &[U32_HEAD, 2, 3 ,4], &[2, 3], &[U32_END];
}

fn main() {
	println!("#1 {:?}", U32_HEAD);
	assert_eq!(U32_HEAD, 255);

	println!("#2 {:?}", U32_END);
	assert_eq!(U32_END, 0);

	//result
	println!("#3 {:?}", U32_ARRAY);
	assert_eq!(U32_ARRAY, [255, 2, 0]);

	println!("#4 {:?}", U32_SARRAY);
	assert_eq!(U32_SARRAY, [255, 2, 3, 4, 2, 3, 0]);
}
```

4. DynGeneric

```rust
#[macro_use]
extern crate cluConstData;

use std::marker::PhantomData;

pub trait TypeTrait {
	const TYPE: &'static str;

	#[inline]
	fn as_type_str() -> &'static str {
		Self::TYPE
	}
}

impl TypeTrait for usize {
	const TYPE: &'static str = "usize";
}


impl TypeTrait for (usize, usize) {
	const_data! {
		const TYPE: &'static str = usize::TYPE, " + ", usize::TYPE;
	}
}

impl TypeTrait for (PhantomData<()>, usize) {
	const_data! {
		const TYPE: &'static str = "PhantomData<()>", " + ", usize::TYPE;
	}
}


fn main() {
	println!("#1 {:?}", usize::as_type_str());
	assert_eq!(usize::as_type_str(), "usize");

	println!("#2 {:?}", <(usize, usize)>::as_type_str());
	assert_eq!(<(usize, usize)>::as_type_str(), "usize + usize");
}
```
*/

#![allow(non_snake_case)]
#![allow(clippy::tabs_in_doc_comments)]
#![no_std]

use cluFullTransmute::unchecked_transmute;
use core::mem::MaybeUninit;

#[cfg_attr(docsrs, doc(cfg(feature = "const_buf")))]
#[cfg(any(test, feature = "const_buf"))]
pub mod buf;

#[cfg_attr(docsrs, doc(cfg(feature = "const_data")))]
#[cfg(any(test, feature = "const_data"))]
mod const_data;

/// Concatenates two arrays into one. (please use `const_data!`` macro)
///
/// # Panics
///
/// The array size is not enough to accommodate two arrays.
#[track_caller]
pub const fn concat_slice_arrays_or_panic<T, const R_LEN: usize>(
	a: &'_ [T],
	b: &'_ [T],
) -> [T; R_LEN]
where
	T: Copy,
{
	/// Internal panic function used for failed contract validation.
	#[track_caller]
	#[inline(never)]
	#[cold]
	const fn _cold_panic(message: &str) -> ! {
		panic!("{}", message)
	}

	let a_len = a.len();
	if R_LEN < (a_len + b.len()) {
		_cold_panic("The array size is not enough to accommodate two arrays.");
	}

	let mut result: [MaybeUninit<T>; R_LEN] = [MaybeUninit::uninit(); R_LEN];

	let mut i = 0usize;
	while a_len > i {
		result[i].write(a[i]);
		i += 1;
	}
	while R_LEN > i {
		result[i].write(b[i - a_len]);
		i += 1;
	}

	// TODO WAIT https://github.com/rust-lang/rust/issues/96097 in stable
	unsafe { unchecked_transmute(result) }
}

/// When `debug_assert` is enabled, the API is checked for correctness
/// (validity of the string in utf-8), in any case it converts the slice array to a string.
///
/// Only for internal use in macros!
#[doc(hidden)]
pub const unsafe fn debug_validate_then_cast_str(array: &[u8]) -> &str {
	debug_assert!(core::str::from_utf8(array).is_ok());

	unsafe { core::str::from_utf8_unchecked(array) }
}

/// Compile-time array concatenation.
///
/// Recursively merges multiple slice-like values (`&[u8]`) **at compile time**,  
/// producing a fixed-size array of type `[u8]`.
///
/// # Examples
/// ```rust
/// use cluConstData::concat_bytes;
/// const A: &[u8] = b"abc";
/// const B: &[u8] = b"def";
/// const FULL: &[u8] = concat_bytes!(A, B, &[b'!']);
/// assert_eq!(&FULL, b"abcdef!");
/// ```
#[macro_export]
macro_rules! concat_bytes {
	[ // end.
		$a: expr $(,)?
	] => {
		$a
	};
	[ // end.
		$a: expr $(,)?
	] => {
		$a
	};
	[$a: expr $(, $b: expr)* $(,)?] => { // [1, 2, 3] => :&[u8] = [1, 2, 3]
		$crate::concat_array! {
			:&[u8] = $a, $($b),*
		}
	};
}

/// Compile-time array concatenation.
///
/// Recursively merges multiple slice-like values (`&[$type]`) **at compile time**,  
/// producing a fixed-size array of type `[$type; N]`.
///
/// # Examples
/// ```rust
/// use cluConstData::concat_array;
/// const A: &[u8] = b"abc";
/// const B: &[u8] = b"def";
/// const FULL: &[u8] = concat_array!(:&[u8] = A, B, &[b'!']);
/// assert_eq!(&FULL, b"abcdef!");
/// ```
#[macro_export]
macro_rules! concat_array {
	[ // end.
		$(:&[$type:ty])? $a: expr $(,)?
	] => {
		$a
	};
	[ // end.
		$(:[$type:ty])? $a: expr $(,)?
	] => {
		$a
	};

	[:[$type:ty] = $a: expr, $b: expr $(,)?] => {{ // &[u8] + &[u8] = [u8; a1.len() + a2.len()]
		const _A_ARRAY: &[$type] = $a;
		const _B_ARRAY: &[$type] = $b;
		const _HIDDEN: [$type; {_A_ARRAY.len() + _B_ARRAY.len()}] = $crate::concat_slice_arrays_or_panic::<
			$type,
			{_A_ARRAY.len() + _B_ARRAY.len()},
		>(_A_ARRAY, _B_ARRAY);

		_HIDDEN
	}};

	[:[$type:ty] = $a: expr $(,$b: expr)+ $(,)?] => {{ // concat array in end
		const _B2: &[$type] = &$crate::concat_array!(:[$type] = $($b),*);
		$crate::concat_array! {
			:[$type] = $a, _B2
		}
	}};

	[:&[$type:ty] = $a: expr $(, $b: expr)* $(,)?] => { // &[u8] + &[u8]
		&$crate::concat_array! {
			:[$type] = $a $(, $b)*
		} as &[_]
	};
}

/// Compile-time string concatenation.
///
/// Recursively concatenates multiple `&'static str` slices **at compile time**,  
/// producing a single `&'static str` result. Useful when you need to stitch together  
/// constant strings in a `const` context—such as inside other macros or when initializing `static` data.
///
/// # Examples
/// ```rust
/// use cluConstData::concat_str;
/// const HELLO: &str = "Hello, ";
/// const MESSAGE: &str = concat_str!(HELLO, "world!");
/// assert_eq!(MESSAGE, "Hello, world!");
/// ```
///
/// # Notes
/// - This macro operates fully at compile time using const evaluations.
#[macro_export]
macro_rules! concat_str {
	[ // end.
		$a: expr $(,)?
	] => {
		$a
	};

	[$a: expr, $b: expr $(,)?] => {{ // &str + &str
		const _A_STR: &[u8] = core::primitive::str::as_bytes($a);
		const _B_STR: &[u8] = core::primitive::str::as_bytes($b);
		const _HIDDEN: &str = unsafe {
			$crate::debug_validate_then_cast_str(
				$crate::concat_array! { // -> &[u8]
					:&[u8] =
						_A_STR,
						_B_STR
				}
			)
		};

		_HIDDEN
	}};

	[$a: expr $(, $b: expr)+ $(,)?] => {{ // concat str in end
		const _STRINEND: &str = $crate::concat_str!($($b),*);
		$crate::concat_str! {
			$a, _STRINEND
		}
	}};
}
