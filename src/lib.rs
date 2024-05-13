//Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 2022
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
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::tabs_in_doc_comments)]
#![no_std]

use core::mem::size_of;

use cluFullTransmute::transmute_or_panic;
mod macros {
	mod const_data;
	mod const_single_data;
}


/// Very coarse concatenation, use safe macros such as 'const_data' !!
pub const unsafe fn const_concat_or_panic<'a, T, const A_LEN: usize, const B_LEN: usize, DataTo>(
	a: &'a [T],
	b: &'a [T]
) -> DataTo where DataTo: 'a, T: Copy {
	#[repr(C, packed)]
	struct __NewDataTo<A, B> {
		a: A,
		b: B,
	}
	if a.len() != A_LEN {
		panic!("Array size argument `A` was entered incorrectly. It is impossible to concat.");
	}
	if b.len() != B_LEN {
		panic!("Array size argument `B` was entered incorrectly. It is impossible to concat.");
	}
	if size_of::<DataTo>() != ((a.len() + b.len()) * size_of::<T>()) {
		panic!("Array size argument `DataTo` was entered incorrectly. It is impossible to concat.");
	}
	
	const fn const_copy_from_slice<T, const N: usize>(slice: &[T]) -> [T; N] where T: Copy {
		let mut rarray: [T; N] = unsafe { core::mem::zeroed() };
		
		let mut i = 0usize;
		let max = slice.len();
		
		while max > i {
			rarray[i] = slice[i];
			i += 1;
		}
		
		rarray
	}
	
	let a: [T; A_LEN] = const_copy_from_slice(a);
	let b: [T; B_LEN] = const_copy_from_slice(b);
	
	let new_array: __NewDataTo<[T; A_LEN], [T; B_LEN]> = __NewDataTo {
		a, // [T; A_LEN]
		b, // [T; B_LEN]
	};
	
	transmute_or_panic(new_array)
	// Transmute result.
	//
	// R<[T; 1024], [T; 1024]> -> [T; 1024 + 1024]
	//
}

/// Raw concatenation, see the description of the macro!
#[doc(hidden)]
#[macro_export]
macro_rules! raw_one_const {
	[$type:ty: $a: expr] => {$a};
	
	[str: $a: expr, $b: expr] => {{
		const _HIDDEN: &'static str = unsafe {
			&*(
				&$crate::raw_one_const! {
					u8:
						$a.as_bytes(), 
						$b.as_bytes()
				} as *const [u8] as *const str
			)
		};
		_HIDDEN
	}};
	
	[str: $a: expr, $($b: expr),*] => {{
		$crate::raw_one_const! {
			str: $a, $crate::raw_one_const!(str: $($b),*)
		}
	}};
	
	[$type:ty: $a: expr, $b: expr] => {{
		const _HIDDEN: [$type; $a.len() + $b.len()] = unsafe {
			$crate::const_concat_or_panic::<
				$type,
				{$a.len()}, {$b.len()},
				
				[$type; $a.len() + $b.len()],
			>($a, $b)
		};
		_HIDDEN
	}};
	
	[$type:ty: $a: expr, $($b: expr),*] => {{
		$crate::raw_one_const! {
			$type: $a, &$crate::raw_one_const!($type: $($b),*)
		}
	}};
}
