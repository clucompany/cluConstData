# cluConstConcat
[![Build Status](https://travis-ci.org/clucompany/cluConstConcat.svg?branch=master)](https://travis-ci.org/clucompany/cluConstConcat)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluConstConcat)](https://crates.io/crates/cluConstConcat)
[![Documentation](https://docs.rs/cluConstConcat/badge.svg)](https://docs.rs/cluConstConcat)

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


4. SingleUse

```
#[macro_use]
extern crate cluConstConcat;

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