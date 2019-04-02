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

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
*/

#![feature(const_fn_union)]
#![feature(untagged_unions)]
#![feature(const_fn)]
#![feature(const_slice_len)]

#[allow(unions_with_drop_fields)]
union UnionTransmute<Value, To> {
	value: Value,
	to: To,
}

impl<A, B> UnionTransmute<A, B> {
	#[inline(always)]
	pub const unsafe fn into(value: A) -> B {
		Self { value }.to
	}
}


#[repr(C)]
#[derive(Debug)]
pub struct ConstConcat<A, B> {
	a: A,
	b: B,
}

impl<DataLeft, DataRight> ConstConcat<DataLeft, DataRight> where DataLeft: Copy, DataRight: Copy {
	pub const unsafe fn const_concat<DataTo, T>(a: &[T], b: &[T]) -> DataTo {
		let result = Self {
			a: *UnionTransmute::<_, &DataLeft>::into(a),
			//Transmute
			//&[T] -> &DataLeft  (DataLeft: &[T; 1024])
			//
			//and copy data!
			//&[T; 1024] -> (a: New [T; 1024] )
			//
			
			b: *UnionTransmute::<_, &DataRight>::into(b),
		};
		//result: 
		//R<DataLeft, DataRight> (R<[T; 1024], [T; 1024]>)
		//
		
		UnionTransmute::into(result)
		//Transmute result.
		//
		//R<[T; 1024], [T; 1024]> -> [T; 1024 + 1024]
		//
	}
}

#[inline(always)]
pub const unsafe fn const_concat<'a, DataLeft, DataRight, DataTo, T>(a: &'a [T], b: &'a [T]) -> DataTo where DataLeft: Copy, DataRight: Copy {
	ConstConcat::<DataLeft, DataRight>::const_concat::<DataTo, T>(a, b)
}


#[macro_export]
macro_rules! const_data {
	//CONCAT!
	[pub const $name: ident : & $l: lifetime [$type: ty] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		pub const $name: & $l [$type] = &$crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	[pub const $name: ident : [$type: ty; $size:expr] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		pub const $name: [$type; $size] = $crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	//NO PUB CONCAT
	[const $name: ident : & $l: lifetime [$type: ty] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		const $name: & $l [$type] = &$crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	[const $name: ident : [$type: ty; $size:expr] = $a:expr, $($b:expr),*;	$($tt:tt)*] => {
		const $name: [$type; $size] = $crate::const_concat!($type: $a $(, $b)*);
		
		$crate::const_data! {$($tt)*}
	};
	
	
	//EMPTY
	[const $name: ident : $ty: ty = $a:expr;			$($tt:tt)*] => {
		const $name: $ty = $a;
		
		$crate::const_data! {$($tt)*}
	};
	
	[pub const $name: ident : $ty: ty = $a:expr;		$($tt:tt)*] => {
		pub const $name: $ty = $a;
		
		$crate::const_data! {$($tt)*}
	};
	
	
	//END
	() => ()
}


#[macro_export]
macro_rules! const_concat {
	[$type: ty: $a: expr, $b: expr] => {
		unsafe {
			$crate::ConstConcat::<[$type; $a.len()], [$type; $b.len()]>::const_concat::<[$type; $a.len() + $b.len()], $type>
			
			($a, $b) 
		}
	};
	
	[@$type: ty : $a: expr, $($b: expr),*] => {{
		const _NO_VISIBLE: [$type] = $crate::const_concat!($type: $($b),*);
		
		$crate::const_concat!($type: $a, _NO_VISIBLE)
	}};
	
	[$type: ty : $a: expr, $($b: expr),*] => {{
		const _NO_VISIBLE: &'static [$type] = &$crate::const_concat!($type: $($b),*);
		
		$crate::const_concat!($type: $a, _NO_VISIBLE)
	}};
	
	
	
	[$a: expr, $($b: expr),*] => {
		$crate::const_concat!(u8: $a, $($b),*)	
	};
	
	[$type:ty: $a: expr] => {$a};
	//Ignore..
	[$a: expr] => {$a};
}
