
use cluFullTransmute::force_transmute;

#[macro_export]
macro_rules! const_array_concat {
	[ $([$type:ty])? $(&[$type2:ty])?: $($a: block)? ] => { // UNK
		$($a)?
	};
	
	[ // MANY ONE_ARRAY
		[$type:ty]: $a: block + $b: block $(+ $all_unk_b: block)+
	] => {
		$crate::const_array_concat! {
			[$type]: { $crate::const_array_concat! {[$type]: {$a} + {$b}} } $(+ $all_unk_b)+
		}
	};
	
	[ // MANY ONE_SLICE_ARRAY
		&[$type2:ty]: $a: block + $b: block $(+ $all_unk_b: block)+
	] => {
		$crate::const_array_concat! {
			&[$type2]: { $crate::const_array_concat! {&[$type2]: $a + $b} } $(+ $all_unk_b)+
		}
	};
	
	[ // ONE_ARRAY
		[$type:ty]: $a: block + $b: block
	] => {{
		#[allow(unused_unsafe)]
		unsafe {
			$crate::array::force_auto_const_concat::<
				[$type; $a.len()], 
				[$type; $b.len()], 
				
				[$type; $a.len() + $b.len()]
			>($a, $b)
		}
	}};
	
	[ // ONE_SLICEARRAY
		&[$type:ty]: $a: block + $b: block
	] => {{
		#[allow(unused_unsafe)]
		unsafe {
			&$crate::array::force_auto_const_slice_concat::<
				'static,
				[$type; $a.len()], 
				[$type; $b.len()], 
				
				[$type; $a.len() + $b.len()],
				$type,
			>($a, $b)
		}
	}};
	
	[ $($unk:tt)+ ] => {
		compile_error!(
			concat!(
				"Unk syntax `const_array_concat` macros, code: `",
				stringify!($($unk)+)
				,"`"
			)
		)
	}
}

#[inline]
pub const unsafe fn force_auto_const_slice_concat<'a, A, B, DataTo, T>(a: &'a [T], b: &'a [T]) -> DataTo where A: 'a + Copy, B: 'a + Copy, DataTo: 'a + Copy {
	#[repr(C)]
	struct __HIDDEN_CONST_CONCAT<A, B> {
		a: A,
		b: B,
	}
	
	let result = __HIDDEN_CONST_CONCAT {
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

#[inline]
pub const unsafe fn force_auto_const_concat<'a, A, B, DataTo>(a: A, b: B) -> DataTo where A: 'a + Copy, B: 'a + Copy, DataTo: 'a + Copy {
	#[repr(C)]
	struct __HIDDEN_CONST_CONCAT<A, B> {
		a: A,
		b: B,
	}
	
	let result = __HIDDEN_CONST_CONCAT {
		a: *(&a as *const A),
		// Transmute
		// &[T] -> &DataLeft  (DataLeft: &[T; 1024])
		//
		// and copy data!
		// &[T; 1024] -> (a: New [T; 1024] )
		//
		
		b: *(&b as *const B),
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

#[cfg(test)]
#[test]
fn test_slicearray_oneconstconcat() {
	const A: &'static [u8] = b"test";
	const B: &'static [u8] = b"1234";
	
	const U: &'static [u8] = const_array_concat!(&[u8]: {A} + {B});
	
	assert_eq!(U, b"test1234");
}

#[cfg(test)]
#[test]
fn test_slicearray_twoconstconcat() {
	const A: &'static [u8] = b"test";
	const B: &'static [u8] = b"1234";
	const C: &'static [u8] = b"567890";
	
	const U: &'static [u8] = const_array_concat!(&[u8]: {A} + {B} + {C} + {C});
	
	assert_eq!(U, b"test1234567890567890");
}


#[cfg(test)]
#[test]
fn test_array_oneconstconcat() {
	const A: [u8; 4] = *b"test";
	const B: [u8; 4] = *b"1234";
	
	const U: [u8; 8] = const_array_concat!([u8]: {A} + {B});
	
	assert_eq!(&U as &[u8], b"test1234" as &[u8]);
}

#[cfg(test)]
#[test]
fn test_array_twoconstconcat() {
	const A: [u8; 4] = *b"test";
	const B: [u8; 4] = *b"1234";
	const C: [u8; 6] = *b"567890";
	
	const fn get_c() -> [u8; 6] {
		C
	}
	const U: [u8; 20] = const_array_concat!([u8]: {A} + {B} + {C} + {get_c()});
	
	assert_eq!(&U as &[u8], b"test1234567890567890" as &[u8]);
}
