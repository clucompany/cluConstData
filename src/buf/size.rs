pub trait ConstByteBufSize {
	const MAX_LEN: usize;
}

impl ConstByteBufSize for usize {
	/// Max decimal digits needed to represent any `usize`.
	const MAX_LEN: usize = match size_of::<usize>() {
		a if a == size_of::<u128>() => <u128 as ConstByteBufSize>::MAX_LEN,
		a if a == size_of::<u64>() => <u64 as ConstByteBufSize>::MAX_LEN,
		a if a == size_of::<u32>() => <u32 as ConstByteBufSize>::MAX_LEN,
		a if a == size_of::<u16>() => <u16 as ConstByteBufSize>::MAX_LEN,
		a if a == size_of::<u8>() => <u8 as ConstByteBufSize>::MAX_LEN,
		_ => <u128 as ConstByteBufSize>::MAX_LEN,
	};
}

impl ConstByteBufSize for u128 {
	/// Max decimal digits needed to represent any `u8`.
	const MAX_LEN: usize = "18446744073709551615".len();
}

impl ConstByteBufSize for u64 {
	/// Max decimal digits needed to represent any `u8`.
	const MAX_LEN: usize = "18446744073709551615".len();
}

impl ConstByteBufSize for u32 {
	/// Max decimal digits needed to represent any `u8`.
	const MAX_LEN: usize = "4294967295".len();
}

impl ConstByteBufSize for u16 {
	/// Max decimal digits needed to represent any `u8`.
	const MAX_LEN: usize = "65535".len();
}

impl ConstByteBufSize for u8 {
	/// Max decimal digits needed to represent any `u8`.
	const MAX_LEN: usize = "255".len();
}

impl ConstByteBufSize for char {
	/// Max decimal digits needed to represent any `char`.
	const MAX_LEN: usize = "4".len();
}
