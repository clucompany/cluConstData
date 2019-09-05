
/// A more advanced version of the concat macro from std, supports constants.
///```
///const A: &'static str = "123";
///const B: &'static str = "456";
///const C: &'static str = "789";
///
///let str = full_concat!(A, B, C, ".");
///assert_eq!(str, "123456789.");
///```
#[macro_export]
macro_rules! full_concat {
	[$e:expr] => ($e);
	[$e:expr, $($a:expr),*] => {{
		const _HIDDEN: &'static str = $crate::raw_one_const!(str: $e, $($a),*);
		_HIDDEN
	}};
	
	[@let $e:expr] => ($e);
	[@let $e:expr, $($a:expr),*] => {{
		let _HIDDEN: &'static str = $crate::raw_one_const!(str: $e, $($a),*);
		_HIDDEN
	}};
}

/// A more advanced version of the concat macro from std, supports constants.
#[macro_export]
macro_rules! concat {
	($($tt:tt)*) => (full_concat!($($tt)*));
}


#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	
	
	#[test]
	fn one_concat_macros() {
		assert_eq!(full_concat!("."), ".");
	}
	#[test]
	fn two_concat_macros() {
		assert_eq!(full_concat!(".", ".."), "...");
	}
	
	#[test]
	fn full_concat_macros() {
		const A: &'static str = "123";
		const B: &'static str = "456";
		const C: &'static str = "789";
		
		let str = full_concat!(A, B, C, ".");
		assert_eq!(str, "123456789.");
	}
}


/*
#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	use std::borrow::Cow;
	use std::ops::Deref;
	
	#[test]
	fn concat_macros() {
		struct Data<A: AsRef<str>> {
			str: A,
		}
		impl<A: AsRef<str>> Data<A> {
			fn new(i: A) -> Self {
				Self {
					str: i
				}	
			}
			fn as_str(&'static self) -> &'static str { 
				self.str.as_ref()
			}
		}
		let data = Data::new("test");
		let data2 = Data::new("test2".to_string());
		
		//let str = crate::concat!(data.as_str(), ".", data2.as_str());
		//let str2 = crate::const_single_data! {
		//	&'static str = data.as_str(), data2.as_str()
		//};	
		
		let a = data.as_str().as_bytes();
		let a2 = data2.as_str().as_bytes();
		
		let str3 = crate::const_concat::<
			[u8; a.len()], 
			[u8; a2.len()],
			u8,
			
			[u8; 
				a.len() + 
				a2.len()
			],
		>(a, a2);
		
		println!("{:?}", str3);
		
		//attempt to use a non-constant value in a constant
		//
		//non-constant value rustc(E0435)
		// :(
}*/
