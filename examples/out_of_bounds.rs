use cluConstData::buf::ConstStrBuf;
use cluConstData::buf::size::ConstByteBufSize;

const PREFIX: &str = "Position ";
const INFIX: &str = "x=";
const MIDFIX: &str = ", y=";
const SUFFIX: &str = " is out of bounds!";

const CAPACITY: usize = PREFIX.len() +
	INFIX.len() +
	usize::MAX_DECIMAL_LEN + // x
	MIDFIX.len() +
	usize::MAX_DECIMAL_LEN + // y
	SUFFIX.len();

const fn make_cstr(x: usize, y: usize) -> ConstStrBuf<CAPACITY> {
	let mut buf = ConstStrBuf::<{ CAPACITY }>::new();

	buf.push_str(PREFIX);
	buf.push_str(INFIX);
	buf.push_usize(x);
	buf.push_str(MIDFIX);
	buf.push_usize(y);
	buf.push_str(SUFFIX);
	buf
}

fn main() {
	let str = make_cstr(1920, 1080);
	assert_eq!(str, "Position x=1920, y=1080 is out of bounds!");
}
