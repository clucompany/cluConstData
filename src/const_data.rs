/// Declarative macro for defining constant data, safely and concisely.
///
/// The `const_data!` macro simplifies the definition of `const` items using expressive syntax  
/// and automatic support for compile-time concatenation. It supports single values, strings, slices, arrays.
///
/// # Features
/// - Concatenates multiple `&'static str` literals at compile time.
/// - Merges slices and arrays (e.g., `[u8]`, `[T; N]`, `&[T]`) using safe const-evaluated logic.
/// - Supports named lifetimes.
/// - Works recursively, allowing multiple constants to be declared in a single block.
/// - Preserves visibility modifiers (`pub`, `pub(crate)`, etc).
///
/// # Supported Patterns
/// | Form                          | Behavior                                             |
/// |-------------------------------|------------------------------------------------------|
/// | `const NAME: T = value;`      | Defines a simple constant value                     |
/// | `const NAME: &str = a, b, c;` | Compile-time concat via `concat_str!`               |
/// | `const NAME: &[$T] = a, b;`   | Compile-time merges slices into new `&[$T]` via `concat_bytes!`  |
/// | `const NAME: [$T; N] = a, b;` | Compile-time merges arrays into new `[T; N]`                     |
/// | `const NAME: &[$T; N] = ...;` | Creates referenced array literal                    |
///
/// # Example
/// ```rust
/// cluConstData::const_data! {
///	pub const HELLO: &str = "Hello, ", "world!";
///	const NUMS: [u8; 5] = &[1, 2], &[3, 4, 5];
///	const FLAGS: &[bool] = &[true], &[false];
/// }
/// ```
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

	// concat_str: &'static str
	[
		$vis:vis const $name: ident : & $l: lifetime str = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name: & $l str = $crate::concat_str!(
			$a, $($b),*
		);

		$crate::const_data! {$($tt)*}
	};
	// concat_str: &str
	[
		$vis:vis const $name: ident : &str = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name: &str = $crate::concat_str!(
			$a, $($b),*
		);

		$crate::const_data! {$($tt)*}
	};

	// concat_bytes: &[T] &[T; N]
	[
		$vis:vis const $name: ident : &$($l: lifetime)? [$t:ty $(; $($_n:expr)?)?] = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name:
			&$($l)? [$t $(; $($_n)?)?] =
			&$crate::concat_bytes!(:[$t] = $a $(, $b)*);

		$crate::const_data! {$($tt)*}
	};
	// concat_bytes: [T] [T; N]
	[
		$vis:vis const $name: ident : [$t:ty $(; $($_n:expr)?)?] = $a:expr, $($b:expr),* $(,)?;

		$($tt:tt)*
	] => {
		$vis const $name:
			[$t $(; $($_n)?)?] = $crate::concat_bytes!(:[$t] = $a $(, $b)*);

		$crate::const_data! {$($tt)*}
	};

	// END
	() => ()
}
