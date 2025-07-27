//! Trait for estimating the maximum decimal length of a type's value.

/// Trait for estimating the maximum decimal length of a type's value.
///
/// This trait provides a const-safe way to determine how many UTF-8 digits (including optional minus sign)  
/// are needed to represent the largest possible value of the type using its decimal form.  
pub trait ConstByteBufSize {
	/// The maximum number of bytes needed to represent this type as decimal string.
	const MAX_DECIMAL_LEN: usize;
}

impl_numconst_buf_size! {
	(usize | u128 | u64 | u32 | u16 | u8)::MAX;
	(isize | i128 | i64 | i32 | i16 | i8)::MIN;
}

impl ConstByteBufSize for char {
	/// Max decimal digits needed to represent any `char`.
	const MAX_DECIMAL_LEN: usize = {
		let mut n = char::MAX as u32;
		let mut count = 0;
		while n != 0 {
			count += 1;
			n /= 10;
		}

		count
	};
}

/// Implements `ConstByteBufSize` for numeric types using either their `.MAX` or `.MIN` value.
///
/// ## Patterns:
/// - `($types)::MAX;` — unsigned types, uses `.MAX`
/// - `($types)::MIN;` — signed types, uses `.MIN` and counts minus sign
///
/// ## Example:
/// ```rust
/// impl_numconst_buf_size! {
///	(u8 | u16 | u32 | usize)::MAX;
///	(i8 | i16 | i32 | isize)::MIN;
/// }
/// ```
macro_rules! impl_numconst_buf_size {
	[
		($($ty:ty)|* ) ::MAX;

		$($all:tt)*
	] => {
		$(
			impl ConstByteBufSize for $ty {
				/// Max decimal digits needed to represent any `
				#[doc = stringify!($ty)]
				#[doc = "`"]
				const MAX_DECIMAL_LEN: usize = {
					let max = <$ty>::MAX;
					let mut n = max;
					let mut count = 0;
					while n != 0 {
						count += 1;
						n /= 10;
					}

					count
				};
			}
		)*

		$crate::buf::size::impl_numconst_buf_size! {
			$($all)*
		}
	};
	[
		($($ty:ty)|* ) ::MIN;

		$($all:tt)*
	] => {
		$(
			impl ConstByteBufSize for $ty {
				/// Max decimal digits needed to represent any signed `
				#[doc = stringify!($ty)]
				#[doc = "`"]
				const MAX_DECIMAL_LEN: usize = {
					let min = <$ty>::MIN;
					let mut n = min.unsigned_abs() as u128;
					let mut count = 1; // знак
					while n != 0 {
						count += 1;
						n /= 10;
					}

					count
				};
			}
		)*

		$crate::buf::size::impl_numconst_buf_size! {
			$($all)*
		}
	};


	() => {}
}

pub(crate) use impl_numconst_buf_size;
