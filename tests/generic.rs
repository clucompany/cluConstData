#[macro_use]
extern crate cluConstData;

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
