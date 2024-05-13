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

/*impl<A, B> TypeTrait for (A, B) where A: TypeTrait, B: TypeTrait {
	const_data! {
		const TYPE: &'static str = A::TYPE, B::TYPE;
	}
}*/
/*
error[E0401]: can't use generic parameters from outer item
  --> examples/dyn_generic.rs:44:30
   |
42 | impl<A, B> TypeTrait for (A, B) where A: TypeTrait, B: TypeTrait {
   |      - type parameter from outer item
43 |     const_data! {
44 |         const TYPE: &'static str = A::TYPE, B::TYPE;
   |                                    ^^^^^^^ use of generic parameter from outer item

*/
//ААААААААААААААААААААААааааааа
//аааааааааааааааааааааааааааааааааааааааа
