# cluConstData
[![Build Status](https://travis-ci.org/clucompany/cluConstData.svg?branch=master)](https://travis-ci.org/clucompany/cluConstData)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluConstData)](https://crates.io/crates/cluConstData)
[![Documentation](https://docs.rs/cluConstData/badge.svg)](https://docs.rs/cluConstData)

Combining any constant data with each other at compile time. Requires a nightly compiler version (not a compiler plugin).

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

impl TypeTrait for usize {
	const_data! {
		const TYPE: &'static str = "usize";
	}
}

impl TypeTrait for u8 {
	const_data! {
		const TYPE: &'static str = "u8";
	}
}

impl TypeTrait for u32 {
	const_data! {
		const TYPE: &'static str = "u32";
	}
}

impl TypeTrait for u64 {
	const_data! {
		const TYPE: &'static str = "u64";
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

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
