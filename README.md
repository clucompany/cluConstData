# cluConstConcat [nightly]
[![Build Status](https://travis-ci.org/clucompany/cluConstConcat.svg?branch=master)](https://travis-ci.org/clucompany/cluConstConcat)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluConstConcat)](https://crates.io/crates/cluConstConcat)
[![Documentation](https://docs.rs/cluConstConcat/badge.svg)](https://docs.rs/cluConstConcat)

# Use

1. Easy

		#![feature(const_slice_len)]

		#[macro_use]
		extern crate cluConstConcat;

		const_data! {
			pub const L_PREFIX:	&'static [u8] = b"<";
			pub const R_PREFIX:	&'static [u8] = b">";

			const MY_DATA:		&'static [u8] = L_PREFIX, b"Test", R_PREFIX;
			const TEST:			[u8; 2] = L_PREFIX, R_PREFIX;
		}

		fn main() {
			println!("L_PREFIX: {:?} \"{}\"", L_PREFIX, unsafe {std::str::from_utf8_unchecked(L_PREFIX)} );
			assert_eq!(L_PREFIX, b"<");

			println!("R_PREFIX: {:?} \"{}\"", R_PREFIX, unsafe {std::str::from_utf8_unchecked(R_PREFIX)} );
			assert_eq!(R_PREFIX, b">");

			println!("MY_DATA: {:?} \"{}\"", MY_DATA, unsafe {std::str::from_utf8_unchecked(MY_DATA)} );
			assert_eq!(MY_DATA, b"<Test>");

			println!("TEST: {:?} \"{}\"", TEST, unsafe {std::str::from_utf8_unchecked(&TEST)} );
			assert_eq!(&TEST, b"<>");
		}
	


# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
