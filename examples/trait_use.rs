
#![feature(const_slice_len)]

#[macro_use]
extern crate cluConstConcat;

use std::marker::PhantomData;

fn main() {
	println!("TypeTrait<usize>: {:?} \"{}\"", usize::RAW_TYPE, unsafe {std::str::from_utf8_unchecked(usize::RAW_TYPE)} );
	assert_eq!(usize::RAW_TYPE, b"usize");
	
	println!("TypeTrait<usize + usize>: {:?} \"{}\"", <(usize, usize)>::RAW_TYPE, unsafe {std::str::from_utf8_unchecked(<(usize, usize)>::RAW_TYPE)} );
	assert_eq!(<(usize, usize)>::RAW_TYPE, b"usize + usize");
	
	println!("TypeTrait<>: \"{}\"", unsafe {std::str::from_utf8_unchecked(
		<(PhantomData<()>, usize)>::RAW_TYPE)
	} );
	
	/*println!("TypeTrait<>: \"{}\"", unsafe {std::str::from_utf8_unchecked(
		<&usize>::RAW_TYPE)
	} );*/
}




pub trait TypeTrait {
	const RAW_TYPE: &'static [u8];
}


//УХ .. :)
//надоело
impl TypeTrait for (usize, usize) {
	const_data! {
		const RAW_TYPE: &'static [u8] = usize::RAW_TYPE, b" + ", usize::RAW_TYPE;
	}
}

impl TypeTrait for (PhantomData<()>, usize) {
	const_data! {
		const RAW_TYPE: &'static [u8] = b"PhantomData<()>", b" ", usize::RAW_TYPE;
	}
}

/*impl<A, B> TypeTrait for (A, B) where A: TypeTrait, B: TypeTrait {
	const_data! {
		const RAW_TYPE: &'static [u8] = A::RAW_TYPE, B::RAW_TYPE;
	}
	/*const_data! {
		const RAW_TYPE: &'static [u8] = &cluConstConcat::const_concat!(u8: b"1", Self::IGNORE);
	}*/
}*/
/*
error[E0599]: no associated item named `RAW_TYPE` found for type `A` in the current scope
  --> examples/trait_use.rs:53:38
   |
53 |         const RAW_TYPE: &'static [u8] = A::RAW_TYPE, B::RAW_TYPE;
   |                                         ---^^^^^^^^
   |                                         |
   |                                         associated item not found in `A`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `RAW_TYPE`, perhaps you need to implement it:
           candidate #1: `TypeTrait`

error[E0599]: no associated item named `RAW_TYPE` found for type `B` in the current scope
  --> examples/trait_use.rs:53:51
   |
53 |         const RAW_TYPE: &'static [u8] = A::RAW_TYPE, B::RAW_TYPE;
   |                                                      ---^^^^^^^^
   |                                                      |
   |                                                      associated item not found in `B`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `RAW_TYPE`, perhaps you need to implement it:
           candidate #1: `TypeTrait`
	     
*/ //ААААААААААААААААААААААааааааа
//аааааааааааааааааааааааааааааааааааааааа


//Возможно ошибка в https://users.rust-lang.org/t/const-associated-item-not-found/23117
//Possible error in https://users.rust-lang.org/t/const-associated-item-not-found/23117


impl TypeTrait for usize {
	const_data! {
		const RAW_TYPE: &'static [u8] = b"usize";
	}
}

impl TypeTrait for u8 {
	const_data! {
		const RAW_TYPE: &'static [u8] = b"u8";
	}
}

impl TypeTrait for u32 {
	const_data! {
		const RAW_TYPE: &'static [u8] = b"u32";
	}
}

impl TypeTrait for u64 {
	const_data! {
		const RAW_TYPE: &'static [u8] = b"u64";
	}
}
