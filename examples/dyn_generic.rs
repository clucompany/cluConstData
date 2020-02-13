
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

/*
impl<A, B> TypeTrait for (A, B) where A: TypeTrait, B: TypeTrait {
	const_data! {
		const TYPE: &'static str = A::TYPE, B::TYPE;
	}
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

