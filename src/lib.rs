//Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 20
//#Ulin Project 1819
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

# License

Copyright 2020 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
*/

#![allow(non_snake_case)]
#![feature(const_fn_trait_bound)]

#![no_std]

#[macro_use]
mod macros {
	#[macro_use]
	mod const_data;
	pub use self::const_data::*;
	
	#[macro_use]
	mod const_single_data;
	pub use self::const_single_data::*;
}
use cluFullTransmute::force_transmute;
pub use self::macros::*;


#[doc(hidden)]
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
			a: *force_transmute::<_, *const A>(a as *const [_]),
			// Transmute
			// &[T] -> &DataLeft  (DataLeft: &[T; 1024])
			//
			// and copy data!
			// &[T; 1024] -> (a: New [T; 1024] )
			//
			
			b: *force_transmute::<_, *const B>(b as *const [_]),
		};
		// result: 
		// R<DataLeft, DataRight> (R<[T; 1024], [T; 1024]>)
		//
		
		force_transmute(result)
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
	pub const unsafe fn const_raw_ptr<'a>(a: &'a [u8]) -> &'a str {
		&*(a as *const [u8] as *const str)
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
		const _HIDDEN: &str = unsafe {
			$crate::ignore_feature::const_raw_ptr(
				&$crate::raw_one_const!{
					u8:
						$a.as_bytes(), 
						$b.as_bytes()
				}
			)
		};
		_HIDDEN
	}};
	
	[str: $a: expr, $($b: expr),*] => {{
		$crate::raw_one_const!(str: $a, $crate::raw_one_const!(str: $($b),*))
	}};
	
	[$type:ty: $a: expr, $b: expr] => {{
		#[allow(unused_unsafe)]
		const _HIDDEN: [$type; $a.len() + $b.len()] = unsafe {
			$crate::const_concat::<
				[$type; $a.len()], 
				[$type; $b.len()],
				$type,
				
				[$type; $a.len() + $b.len()],
			>($a, $b)
		};
		_HIDDEN
	}};
	
	[$type:ty: $a: expr, $($b: expr),*] => {{
		$crate::raw_one_const!($type: $a, &$crate::raw_one_const!($type: $($b),*))
	}};
	
	
}
