#[cfg(any(test, feature = "const_data"))]
mod test_const_ata {
	use cluConstData::const_data;

	#[test]
	fn single_const_data() {
		const_data! {
			const A: &[u8] = b"123";
			const B: &'static str = "123";
			const C: u32 = 10;
		}

		assert_eq!(A, b"123");
		assert_eq!(B, "123");
		assert_eq!(C, 10);
	}

	#[test]
	fn u8_array_const_data() {
		const_data! {
			const A: &'static [u8] = b"123";
			const B: &[u8] = b".end";

			const ARRAY: &'static [u8] = A, B, b"1234";
			const ARRAY2: &[u8] = ARRAY, b"1234", b".";
		}

		assert_eq!(A, b"123");
		assert_eq!(B, b".end");

		assert_eq!(ARRAY, b"123.end1234");
		assert_eq!(ARRAY2, b"123.end12341234.");
	}

	#[test]
	fn str_array_const_data() {
		const_data! {
			const A: &'static str = "123";
			const B: &'static str = ".end";

			const ARRAY: &'static str = A, B, "1234";
			const ARRAY2: &[u8] = ARRAY.as_bytes(), "1234".as_bytes(), ".".as_bytes();
		}

		assert_eq!(A, "123");
		assert_eq!(B, ".end");

		assert_eq!(ARRAY, "123.end1234");
		assert_eq!(ARRAY2, b"123.end12341234.");
	}

	#[test]
	fn generic_test() {
		trait AGeneric {
			const STR: &'static str;

			#[inline]
			fn as_str() -> &'static str {
				Self::STR
			}
		}
		struct A;
		struct B;

		impl AGeneric for A {
			const STR: &'static str = "A";
		}
		impl AGeneric for B {
			const STR: &'static str = "B";
		}

		impl AGeneric for (A, B) {
			const_data! {
				const STR: &'static str = A::STR, " + ", B::STR;
			}
		}

		assert_eq!(<(A, B)>::as_str(), "A + B");
	}
}
