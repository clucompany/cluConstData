pub mod size;

use core::marker::PhantomData;
use core::mem::MaybeUninit;

use crate::buf::size::ConstByteBufSize;

pub type ConstStrBuf<const CAP: usize> = ConstByteBuf<CAP, Utf8SafeBuf>;

/// Fixed-capacity builder for `const` contexts.
///
/// Allows appending strings, raw bytes, or `usize` in decimal form—all in `const fn`.
/// Internally uses a `[u8; CAP]` buffer and tracks the current write position.
pub struct ConstByteBuf<const CAP: usize, TData = DefBuf>
where
	TData: ConstByteBufData,
{
	tdata: PhantomData<TData>,

	buf: [MaybeUninit<u8>; CAP],
	pos: usize,
}

pub trait ConstByteBufData {}

pub enum Utf8SafeBuf {}
impl ConstByteBufData for Utf8SafeBuf {}

pub enum DefBuf {}
impl ConstByteBufData for DefBuf {}

impl<const CAP: usize, TData: ConstByteBufData> ConstByteBuf<CAP, TData> {
	/// Creates an empty builder (buffer zeroed, position = 0).
	#[inline]
	pub const fn new() -> Self {
		Self {
			tdata: PhantomData,

			buf: [MaybeUninit::uninit(); CAP],
			pos: 0,
		}
	}

	#[inline]
	pub const fn clone(&self) -> Self {
		Self {
			tdata: PhantomData,

			buf: self.buf,
			pos: self.pos,
		}
	}

	/// Resets write position to 0, retains buffer contents.
	#[inline]
	pub const fn clear(&mut self) {
		self.pos = 0;
	}

	/// Total capacity in bytes.
	#[inline]
	pub const fn capacity(&self) -> usize {
		CAP
	}

	/// Number of bytes already written.
	#[inline]
	pub const fn len(&self) -> usize {
		self.pos
	}

	/// Determine if a recording has been made previously
	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.pos == 0
	}

	/// Available capacity.
	#[inline]
	pub const fn available(&self) -> usize {
		CAP - self.pos
	}

	/// Returns a raw pointer to the slice's buffer.
	#[inline]
	pub const fn as_ptr(&self) -> *const MaybeUninit<u8> {
		self.buf.as_ptr()
	}

	/// Returns written bytes as a slice.
	#[inline]
	pub const fn as_bytes(&self) -> &[u8] {
		// SAFETY: we only write valid UTF-8 bytes via `write_str` and `write_usize`
		unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.pos) }
	}

	/// Appends a UTF-8 string.
	#[inline]
	pub const fn push_str(&mut self, s: &str) -> usize {
		// SAFETY: `s.as_bytes()` length checked in `write_bytes_unchecked`
		self.__write_bytes_unchecked(s.as_bytes())
	}

	/// Appends raw bytes without UTF-8 check. Panics on overflow.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences, if you send non-`utf-8` sequences you just break the API.
	#[track_caller]
	#[inline]
	const fn __write_bytes_unchecked(&mut self, data: &[u8]) -> usize {
		let datalen = data.len();
		if self.pos + datalen > CAP {
			Self::cold_panic("ConstByteBuf overflow: capacity exceeded");
		}

		let mut i = 0;
		while i < datalen {
			self.buf[self.pos + i].write(data[i]);
			i += 1;
		}
		self.pos += datalen;
		datalen
	}

	/// Appends byte. Panics on overflow.
	const fn __write_byte(&mut self, data: u8) -> usize {
		let datalen = 1;
		if self.pos + datalen > CAP {
			Self::cold_panic("ConstByteBuf overflow: capacity exceeded");
		}

		self.buf[self.pos].write(data);
		self.pos += datalen;
		datalen
	}

	/// Appends any symbol
	pub const fn push_char(&mut self, value: char) -> usize {
		let mut buf: [u8; <char as ConstByteBufSize>::MAX_DECIMAL_LEN] =
			unsafe { core::mem::zeroed() };
		let str = value.encode_utf8(&mut buf);

		self.push_str(str)
	}

	/// Appends decimal representation of `usize`.
	pub const fn push_usize(&mut self, mut value: usize) -> usize {
		let mut arr: [MaybeUninit<u8>; usize::MAX_DECIMAL_LEN] =
			[MaybeUninit::uninit(); usize::MAX_DECIMAL_LEN];
		let arr_len = arr.len();
		let mut len = 0;

		if value == 0 {
			arr[arr_len - 1].write(b'0');
			len = 1;
		} else {
			let mut i = arr_len;
			while value != 0 {
				i -= 1;

				arr[i].write(b'0' + (value % 10) as u8);
				value /= 10;
				len += 1;
			}
			// Move slice to start of tmp
			// idx now points to first used digit
		}

		let start = arr_len - len;
		let dataptr = unsafe { arr.as_ptr().add(start) };
		let slice = unsafe { core::slice::from_raw_parts(dataptr as *const u8, len) };

		self.__write_bytes_unchecked(slice)
	}

	#[cold]
	#[track_caller]
	#[inline(never)]
	const fn cold_panic(data: &str) -> ! {
		panic!("{}", data);
	}
}

impl<const CAP: usize> ConstByteBuf<CAP, Utf8SafeBuf> {
	/// Returns written bytes as `&str`. Debug-asserts valid UTF-8.
	#[inline]
	pub const fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
	}

	/// Appends raw bytes without UTF-8 check. Panics on overflow.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences, if you send non-`utf-8` sequences you just break the API.
	#[track_caller]
	#[inline]
	pub const unsafe fn write_bytes_unchecked(&mut self, data: &[u8]) -> usize {
		self.__write_bytes_unchecked(data)
	}

	/// Appends byte. Panics on overflow.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences, if you send non-`utf-8` sequences you just break the API.
	#[inline]
	pub const unsafe fn write_byte(&mut self, data: u8) -> usize {
		self.__write_byte(data)
	}
}

impl<const CAP: usize> ConstByteBuf<CAP, DefBuf> {
	/// Appends raw bytes. Panics on overflow.
	///
	#[track_caller]
	#[inline]
	pub const fn write_bytes(&mut self, data: &[u8]) -> usize {
		self.__write_bytes_unchecked(data)
	}

	/// Appends byte. Panics on overflow.
	pub const fn write_byte(&mut self, data: u8) -> usize {
		self.__write_byte(data)
	}
}

impl<const CAP: usize, TData> Clone for ConstByteBuf<CAP, TData>
where
	TData: ConstByteBufData,
{
	#[inline]
	fn clone(&self) -> Self {
		ConstByteBuf::clone(self)
	}
}

impl<const CAP: usize, TData> Default for ConstByteBuf<CAP, TData>
where
	TData: ConstByteBufData,
{
	#[inline]
	fn default() -> Self {
		ConstByteBuf::new()
	}
}
