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

	// concat_const_array: &[T] &[T; N]
	[
		$vis:vis const $name: ident : &$($l: lifetime)? [$t:ty $(; $($_n:expr)?)?] = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name:
			&$($l)? [$t $(; $($_n)?)?] =
			&$crate::concat_const_array!(:[$t] = $a $(, $b)*);

		$crate::const_data! {$($tt)*}
	};
	// concat_const_array: [T] [T; N]
	[
		$vis:vis const $name: ident : [$t:ty $(; $($_n:expr)?)?] = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name:
			[$t $(; $($_n)?)?] = $crate::concat_const_array!(:[$t] = $a $(, $b)*);

		$crate::const_data! {$($tt)*}
	};

	//END
	() => ()
}
