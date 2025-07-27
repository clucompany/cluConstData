use cluConstData::buf::ConstStrBuf;
use cluConstData::buf::size::ConstByteBufSize;

#[test]
fn concat_writer_nums() {
	let mut u_writer = ConstStrBuf::<{ usize::MAX_LEN }>::new();
	let u_sizeof = size_of::<usize>();

	if size_of::<u128>() <= u_sizeof {
		// machine u64 mode
		u_writer.push_usize(u128::MAX as _);
		assert!(core::str::from_utf8(u_writer.as_bytes()).is_ok());
		assert_eq!(u_writer.as_str(), "340282366920938463463374607431768211455");

		if size_of::<u128>() == u_sizeof {
			assert_eq!(u_writer.available(), 0);
		}
		u_writer.clear();
	}
	if size_of::<u64>() <= u_sizeof {
		// machine u64 mode
		u_writer.push_usize(u64::MAX as _);
		assert!(core::str::from_utf8(u_writer.as_bytes()).is_ok());
		assert_eq!(u_writer.as_str(), "18446744073709551615");
		if size_of::<u64>() == u_sizeof {
			assert_eq!(u_writer.available(), 0);
		}
		u_writer.clear();
	}
	if size_of::<u32>() <= u_sizeof {
		// machine u32 mode
		u_writer.push_usize(u32::MAX as _);
		assert!(core::str::from_utf8(u_writer.as_bytes()).is_ok());
		assert_eq!(u_writer.as_str(), "4294967295");
		if size_of::<u32>() == u_sizeof {
			assert_eq!(u_writer.available(), 0);
		}
		u_writer.clear();
	}
	if size_of::<u16>() <= u_sizeof {
		// machine u16 mode
		u_writer.push_usize(u16::MAX as _);
		assert!(core::str::from_utf8(u_writer.as_bytes()).is_ok());
		assert_eq!(u_writer.as_str(), "65535");
		if size_of::<u16>() == u_sizeof {
			assert_eq!(u_writer.available(), 0);
		}
		u_writer.clear();
	}
	if size_of::<u8>() <= u_sizeof {
		// machine u16 mode
		u_writer.push_usize(u8::MAX as _);
		assert!(core::str::from_utf8(u_writer.as_bytes()).is_ok());
		assert_eq!(u_writer.as_str(), "255");
		if size_of::<u8>() == u_sizeof {
			assert_eq!(u_writer.available(), 0);
		}
		u_writer.clear();
	}

	// zero
	u_writer.push_usize(0 as _);
	assert_eq!(u_writer.as_str(), "0");
	u_writer.clear();
	assert_eq!(u_writer.as_str(), "");
}

#[test]
fn concat_writer_str() {
	let a_size = 1024;
	let b_size = 1025;

	// format_test
	//
	// format!(
	//	Error using `transmute`, size of type A={} is not equal to size of type B={}.
	// )
	const S0: &str = "Error using `transmute`, size of type A=";
	const S1: &str = " is not equal to size of type B=";

	let mut concat = ConstStrBuf::<
		{ S0.len() + usize::MAX_LEN + S1.len() + usize::MAX_LEN + char::MAX_LEN },
	>::new();
	concat.push_str(S0);
	concat.push_usize(a_size);
	concat.push_str(S1);
	concat.push_usize(b_size);
	concat.push_char('.');

	assert!(core::str::from_utf8(concat.as_bytes()).is_ok());
	assert_eq!(
		concat.as_str(),
		"Error using `transmute`, size of type A=1024 is not equal to size of type B=1025.",
	);
}
