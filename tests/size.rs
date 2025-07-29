#[cfg(any(test, feature = "const_data"))]
mod test_size {
	use cluConstData::buf::size::ConstByteBufSize;

	#[test]
	fn test_sizealltypes() {
		macro_rules! __codegen_test_sizealltypes {
			[
				( $($ty:ty),* )::MAX;
			] => {
				$(
					assert!(<$ty as ConstByteBufSize>::MAX_DECIMAL_LEN == <$ty>::MAX.to_string().len());
				)*
			};
			[
				( $($ty:ty),* )::MIN;
			] => {
				$(
					assert!(<$ty as ConstByteBufSize>::MAX_DECIMAL_LEN == <$ty>::MIN.to_string().len());
				)*
			};
		}

		__codegen_test_sizealltypes!(
			(usize, u128, u64, u32, u16, u8)::MAX;
		);
		__codegen_test_sizealltypes!(
			(isize, i128, i64, i32, i16, i8)::MIN;
		);
		assert!(
			<char as ConstByteBufSize>::MAX_DECIMAL_LEN == (<char>::MAX as u32).to_string().len()
		);
	}
}
