///
/// The safe and recommended method of the description of constant data.
#[macro_export]
macro_rules! const_data {
	// single value
	[
		$vis:vis const $name: ident : $ty: ty = $a:expr $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name: $ty = $a;

		$crate::const_data! {$($tt)*}
	};

	// concat_const_str: &'static str
	[
		$vis:vis const $name: ident : & $l: lifetime str = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name: & $l str = $crate::concat_const_str!(
			$a, $($b),*
		);

		$crate::const_data! {$($tt)*}
	};
	// concat_const_str: &str
	[
		$vis:vis const $name: ident : &str = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name: &str = $crate::concat_const_str!(
			$a, $($b),*
		);

		$crate::const_data! {$($tt)*}
	};

	// concat_const_slicearray: &[T] &[T; N]
	[
		$vis:vis const $name: ident : &$($l: lifetime)? [$t:ty $(; $($_n:expr)?)?] = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name:
			&$($l)? [$t $(; $($_n)?)?] =
			&$crate::concat_const_slicearray!([$t]: $a $(, $b)*);

		$crate::const_data! {$($tt)*}
	};
	// concat_const_slicearray: [T] [T; N]
	[
		$vis:vis const $name: ident : [$t:ty $(; $($_n:expr)?)?] = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name:
			[$t $(; $($_n)?)?] = $crate::concat_const_slicearray!([$t]: $a $(, $b)*);

		$crate::const_data! {$($tt)*}
	};

	//END
	() => ()
}

#[test]
#[cfg(test)]
fn one_const_data() {
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
#[cfg(test)]
fn u8_array_const_data() {
	const_data! {
		const A: &'static [u8] = b"123";
		const B: &'static [u8] = b".end";

		const ARRAY: &'static [u8] = A, B, b"1234";
		const ARRAY2: &'static [u8] = ARRAY, b"1234", b".";
	}

	assert_eq!(A, b"123");
	assert_eq!(B, b".end");

	assert_eq!(ARRAY, b"123.end1234");
	assert_eq!(ARRAY2, b"123.end12341234.");
}

#[test]
#[cfg(test)]
fn str_array_const_data() {
	const_data! {
		const A: &'static str = "123";
		const B: &'static str = ".end";

		const ARRAY: &'static str = A, B, "1234";
		const ARRAY2: &'static str = ARRAY, "1234", ".";
	}

	assert_eq!(A, "123");
	assert_eq!(B, ".end");

	assert_eq!(ARRAY, "123.end1234");
	assert_eq!(ARRAY2, "123.end12341234.");
}
