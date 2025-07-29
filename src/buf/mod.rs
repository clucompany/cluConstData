//! Compile-time buffer builder with UTF-8 safety and decimal formatting.
//!

pub mod size;

use crate::buf::size::ConstByteBufSize;
use cluFullTransmute::unchecked_transmute;
use core::fmt::Debug;
use core::fmt::Display;
use core::hash::Hash;
use core::marker::PhantomData;
use core::mem::MaybeUninit;

/// UTF-8 safe const buffer builder.
///
/// # Example
/// ```rust
/// use cluConstData::buf::ConstStrBuf;
/// const fn build_name() -> ConstStrBuf<16> {
///	let mut buf = ConstStrBuf::<16>::new();
///	buf.push_str("hello");
///	buf.push_char('!');
///	buf
/// }
/// ```
///
/// Suitable for compile-time generation of valid strings.
pub type ConstStrBuf<const CAP: usize> = ConstByteBuf<CAP, Utf8SafeBuf>;

/// Fixed-capacity builder for `const` contexts.
///
/// Allows appending strings, raw bytes, or `usize` in decimal form—all in `const fn`.
/// Internally uses a `[MaybeUninit<u8>; CAP]` buffer and tracks the current write position.
pub struct ConstByteBuf<const CAP: usize, TData = DefBuf>
where
	TData: ConstByteBufData,
{
	tdata: PhantomData<TData>,

	buf: [MaybeUninit<u8>; CAP],
	wpos: usize,
}

/// Marker trait for buffer behavior customization.
pub trait ConstByteBufData {}

/// Marker type enforcing UTF-8 validation.
pub enum Utf8SafeBuf {}
impl ConstByteBufData for Utf8SafeBuf {}

/// Marker type allowing unrestricted raw byte access.
///
/// Grants access to raw byte writing methods like `write_bytes`, which bypass UTF-8 checks.
pub enum DefBuf {}
impl ConstByteBufData for DefBuf {}

impl<const CAP: usize, TData: ConstByteBufData> ConstByteBuf<CAP, TData> {
	/// Creates a new empty buffer.
	///
	/// Initializes all memory to uninitialized (`MaybeUninit`),
	/// with the write cursor set to `0`.
	#[inline]
	pub const fn new() -> Self {
		Self {
			tdata: PhantomData,

			buf: [MaybeUninit::uninit(); CAP],
			wpos: 0,
		}
	}

	/// Creates an exact copy of the buffer.
	#[inline]
	pub const fn clone(&self) -> Self {
		Self {
			tdata: PhantomData,

			buf: self.buf,
			wpos: self.wpos,
		}
	}

	/// Removes and returns the last written byte.
	const fn _pop(&mut self) -> Option<u8> {
		match self.wpos {
			0 => None,
			pos => {
				let result = core::mem::replace(&mut self.buf[pos], MaybeUninit::uninit());
				self.wpos -= 1;

				Some(unsafe { result.assume_init() })
			}
		}
	}

	/// Converts this `ConstStrBuf` into a fully initialized `[u8; CAP]` array,
	/// filling remaining capacity with a custom trailing byte (`space`).
	const fn _into_array(mut self, space: u8) -> (usize, [u8; CAP]) {
		let len = self.len();
		while self.__try_write_byte(space).is_ok() {} // utf-8 safe

		// TODO WAIT https://github.com/rust-lang/rust/issues/96097 in stable
		(len, unsafe {
			unchecked_transmute(self.buf as [MaybeUninit<u8>; CAP])
		})
	}

	/// Resets write position to 0, retains buffer contents.
	#[inline]
	pub const fn clear(&mut self) {
		self.wpos = 0;
	}

	/// Total capacity in bytes.
	#[inline]
	pub const fn capacity(&self) -> usize {
		self.buf.len()
	}

	/// Returns a mutable reference to the byte at the given position.
	const fn _get_mut(&mut self, pos: usize) -> Option<&mut u8> {
		if pos > self.wpos {
			return None;
		}

		unsafe { Some(self.buf[pos].assume_init_mut()) }
	}

	/// Returns an immutable reference to the byte at the given position.
	pub const fn get(&self, pos: usize) -> Option<&u8> {
		if pos > self.wpos {
			return None;
		}

		unsafe { Some(self.buf[pos].assume_init_ref()) }
	}

	/// Number of bytes already written.
	#[inline]
	pub const fn len(&self) -> usize {
		self.wpos
	}

	/// Determine if a recording has been made previously
	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.wpos == 0
	}

	/// Available capacity.
	#[inline]
	pub const fn available(&self) -> usize {
		CAP - self.wpos
	}

	/// Returns a raw pointer to the slice's buffer.
	#[inline]
	pub const fn as_ptr(&self) -> *const MaybeUninit<u8> {
		self.buf.as_ptr()
	}

	/// Returns a raw mut pointer to the slice's buffer.
	#[inline]
	pub const fn as_mut_ptr(&mut self) -> *mut MaybeUninit<u8> {
		self.buf.as_mut_ptr()
	}

	/// Returns a slice of written bytes.
	#[inline]
	pub const fn as_bytes(&self) -> &[u8] {
		unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.wpos) }
	}

	/// Returns a mut slice of written bytes.
	#[inline]
	const fn _as_mut_bytes(&mut self) -> &mut [u8] {
		unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut u8, self.wpos) }
	}

	/// Appends a UTF-8 string.
	///
	/// Panics on overflow.
	#[inline]
	pub const fn push_str(&mut self, s: &str) -> usize {
		self.__write_bytes_unchecked(s.as_bytes())
	}

	/// Appends a UTF-8 string.
	#[inline]
	pub const fn try_push_str(&mut self, s: &str) -> Result<usize, StackOverflow> {
		self.__try_write_bytes_unchecked(s.as_bytes())
	}

	/// Appends raw bytes without UTF-8 check.
	///
	/// Panics on overflow.
	#[track_caller]
	#[inline]
	const fn __write_bytes_unchecked(&mut self, data: &[u8]) -> usize {
		match self.__try_write_bytes_unchecked(data) {
			Ok(a) => a,
			Err(_) => Self::cold_overflow_panic(),
		}
	}

	/// Appends raw bytes without UTF-8 check. Panics on overflow.
	const fn __try_write_bytes_unchecked(&mut self, data: &[u8]) -> Result<usize, StackOverflow> {
		let datalen = data.len();
		if self.wpos + datalen > CAP {
			return Err(StackOverflow);
		}

		let mut i = 0;
		while i < datalen {
			self.buf[self.wpos + i].write(data[i]);
			i += 1;
		}
		self.wpos += datalen;
		Ok(datalen)
	}

	/// Appends byte.
	///
	/// Panics on overflow.
	const fn __write_byte(&mut self, data: u8) -> usize {
		match self.__try_write_byte(data) {
			Ok(a) => a,
			Err(_) => Self::cold_overflow_panic(),
		}
	}

	/// Appends byte.
	const fn __try_write_byte(&mut self, data: u8) -> Result<usize, StackOverflow> {
		let datalen = 1;
		if self.wpos + datalen > CAP {
			return Err(StackOverflow);
		}

		self.buf[self.wpos].write(data);
		self.wpos += datalen;
		Ok(datalen)
	}

	/// Appends a single UTF-8 character.
	///
	/// Panics on overflow.
	pub const fn push_char(&mut self, value: char) -> usize {
		match self.try_push_char(value) {
			Ok(a) => a,
			Err(_) => Self::cold_overflow_panic(),
		}
	}

	/// Appends a single UTF-8 character.
	pub const fn try_push_char(&mut self, value: char) -> Result<usize, StackOverflow> {
		let mut buf: [u8; <char as ConstByteBufSize>::MAX_DECIMAL_LEN] =
			unsafe { core::mem::zeroed() };
		let str = value.encode_utf8(&mut buf);

		self.try_push_str(str)
	}

	/// Appends decimal representation of `usize`.
	///
	/// Panics on overflow.
	pub const fn push_usize(&mut self, value: usize) -> usize {
		match self.try_push_usize(value) {
			Ok(a) => a,
			Err(_) => Self::cold_overflow_panic(),
		}
	}

	/// Appends decimal representation of `usize`.
	pub const fn try_push_usize(&mut self, mut value: usize) -> Result<usize, StackOverflow> {
		let mut arr: [MaybeUninit<u8>; usize::MAX_DECIMAL_LEN] =
			[MaybeUninit::uninit(); usize::MAX_DECIMAL_LEN];
		let arr_len = arr.len();
		let mut len;

		if value == 0 {
			arr[arr_len - 1].write(b'0');
			len = 1;
		} else {
			len = 0;
			let mut i = arr_len;
			while value != 0 {
				i -= 1;

				arr[i].write(b'0' + (value % 10) as u8);
				value /= 10;
				len += 1;
			}
		}

		let start = arr_len - len;
		let dataptr = unsafe { arr.as_ptr().add(start) };
		let slice = unsafe { core::slice::from_raw_parts(dataptr as *const u8, len) };

		self.__try_write_bytes_unchecked(slice)
	}

	/// Appends decimal representation of `isize`.
	///
	/// Panics on overflow.
	pub const fn push_isize(&mut self, value: isize) -> usize {
		match self.try_push_isize(value) {
			Ok(a) => a,
			Err(_) => Self::cold_overflow_panic(),
		}
	}

	/// Appends decimal representation of `isize`.
	pub const fn try_push_isize(&mut self, value: isize) -> Result<usize, StackOverflow> {
		let abs: usize = match value < 0 {
			true => {
				if let Err(e) = self.__try_write_byte(b'-') {
					return Err(e);
				}

				value.wrapping_neg() as usize
			}
			false => value as usize,
		};

		self.try_push_usize(abs)
	}

	/// Panics when a `ConstByteBuf` overflows its allocated capacity.
	///
	/// This function is marked as `#[cold]` and `#[inline(never)]` to ensure
	/// it remains outside hot execution paths and does not interfere with performance.
	/// Additionally, `#[track_caller]` preserves the location of the original call site,
	/// helping diagnose overflows during compile-time or runtime evaluation.
	#[cold]
	#[track_caller]
	#[inline(never)]
	const fn cold_overflow_panic() -> ! {
		panic!("ConstByteBuf overflow: capacity exceeded");
	}
}

impl<const CAP: usize> ConstByteBuf<CAP, Utf8SafeBuf> {
	/// Returns a slice of written bytes as a UTF-8 string.
	#[inline]
	pub const fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
	}

	/// Returns a slice of written bytes as a UTF-8 string.
	#[inline]
	pub const fn as_mut_str(&mut self) -> &mut str {
		unsafe { core::str::from_utf8_unchecked_mut(self.as_mut_bytes()) }
	}

	/// Appends raw bytes without UTF-8 check.
	///
	/// Panics on overflow.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences,
	/// if you send non-`utf-8` sequences you just break the API.
	#[track_caller]
	#[inline]
	pub const unsafe fn write_bytes_unchecked(&mut self, data: &[u8]) -> usize {
		self.__write_bytes_unchecked(data)
	}

	/// Appends raw bytes without UTF-8 check. Panics on overflow.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences,
	/// if you send non-`utf-8` sequences you just break the API.
	#[inline]
	pub const unsafe fn try_write_bytes_unchecked(
		&mut self,
		data: &[u8],
	) -> Result<usize, StackOverflow> {
		self.__try_write_bytes_unchecked(data)
	}

	/// Appends byte.
	///
	/// Panics on overflow.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences,
	/// if you send non-`utf-8` sequences you just break the API.
	#[inline]
	pub const unsafe fn write_byte(&mut self, data: u8) -> usize {
		self.__write_byte(data)
	}

	/// Appends byte.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences,
	/// if you send non-`utf-8` sequences you just break the API.
	#[inline]
	pub const unsafe fn try_write_byte(&mut self, data: u8) -> Result<usize, StackOverflow> {
		self.__try_write_byte(data)
	}

	/// Returns a mutable reference to the byte at the given position.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences,
	/// if you send non-`utf-8` sequences you just break the API.
	#[inline]
	pub const unsafe fn get_mut(&mut self, pos: usize) -> Option<&mut u8> {
		self._get_mut(pos)
	}

	/// Removes and returns the last written byte.
	///
	/// # Safety
	/// May break UTF-8
	#[inline]
	pub const unsafe fn pop(&mut self) -> Option<u8> {
		self._pop()
	}

	/// Returns a mut slice of written bytes.
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences,
	/// if you send non-`utf-8` sequences you just break the API.
	#[inline]
	pub const unsafe fn as_mut_bytes(&mut self) -> &mut [u8] {
		self._as_mut_bytes()
	}

	/// Converts this `ConstStrBuf` into a fully initialized `[u8; CAP]` array,
	/// filling remaining capacity with a custom trailing byte (0)
	/// and also returning its original length.
	#[inline]
	pub const fn into_array_filled_with_zero(self) -> (usize, [u8; CAP]) {
		self._into_array(b' ') // utf-8 safe
	}

	/// Converts this `ConstStrBuf` into a fully initialized `[u8; CAP]` array,
	/// filling remaining capacity with a custom trailing byte (b' ')
	/// and also returning its original length.
	#[inline]
	pub const fn into_array_filled_with_space(self) -> (usize, [u8; CAP]) {
		self._into_array(b' ') // utf-8 safe
	}

	/// Converts this ConstStrBuf into a fully initialized [u8; CAP] array,
	/// filling remaining capacity with a custom trailing byte (space).
	///
	/// # Safety
	/// It's safe as long as you send `utf-8` sequences,
	/// if you send non-`utf-8` sequences you just break the API.
	#[inline]
	pub const unsafe fn into_array(self, space: u8) -> (usize, [u8; CAP]) {
		self._into_array(space)
	}
}

impl<const CAP: usize> ConstByteBuf<CAP, DefBuf> {
	/// Appends raw bytes. Panics on overflow.
	///
	/// Panics on overflow.
	#[track_caller]
	#[inline]
	pub const fn write_bytes(&mut self, data: &[u8]) -> usize {
		self.__write_bytes_unchecked(data)
	}

	/// Appends raw bytes.
	///
	#[inline]
	pub const fn try_write_bytes(&mut self, data: &[u8]) -> Result<usize, StackOverflow> {
		self.__try_write_bytes_unchecked(data)
	}

	/// Appends byte.
	///
	/// Panics on overflow.
	#[track_caller]
	#[inline]
	pub const fn write_byte(&mut self, data: u8) -> usize {
		self.__write_byte(data)
	}

	/// Appends byte.
	#[inline]
	pub const fn try_write_byte(&mut self, data: u8) -> Result<usize, StackOverflow> {
		self.__try_write_byte(data)
	}

	/// Returns a mutable reference to the byte at the given position.
	#[inline]
	pub const fn get_mut(&mut self, pos: usize) -> Option<&mut u8> {
		self._get_mut(pos)
	}

	/// Removes and returns the last written byte.
	#[inline]
	pub const fn pop(&mut self) -> Option<u8> {
		self._pop()
	}

	/// Returns a mut slice of written bytes.
	#[inline]
	pub const fn as_mut_bytes(&mut self) -> &mut [u8] {
		self._as_mut_bytes()
	}

	/// Converts this ConstStrBuf into a fully initialized [u8; CAP] array,
	/// filling remaining capacity with a custom trailing byte (space)
	/// and also returning its original length.
	#[inline]
	pub const fn into_array(self, space: u8) -> (usize, [u8; CAP]) {
		self._into_array(space) // utf-8 safe
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

impl<const CAP: usize> Display for ConstByteBuf<CAP, Utf8SafeBuf> {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		Display::fmt(self.as_str(), f)
	}
}

impl<const CAP: usize, TData> Debug for ConstByteBuf<CAP, TData>
where
	TData: ConstByteBufData,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("ConstByteBuf")
			.field("buf", &self.as_bytes())
			.field("wpos", &self.wpos)
			.finish()
	}
}

impl<const CAP: usize, TData> Eq for ConstByteBuf<CAP, TData> where TData: ConstByteBufData {}

impl<const CAP: usize, TData> PartialEq for ConstByteBuf<CAP, TData>
where
	TData: ConstByteBufData,
{
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		PartialEq::eq(self.as_bytes(), other.as_bytes())
	}
}

impl<const CAP: usize> PartialEq<str> for ConstByteBuf<CAP, Utf8SafeBuf> {
	#[inline]
	fn eq(&self, other: &str) -> bool {
		PartialEq::eq(self.as_str(), other)
	}
}

impl<const CAP: usize> PartialEq<&'_ str> for ConstByteBuf<CAP, Utf8SafeBuf> {
	#[inline]
	fn eq(&self, other: &&str) -> bool {
		PartialEq::eq(self.as_str(), *other)
	}
}

impl<const CAP: usize, TData> PartialEq<[u8]> for ConstByteBuf<CAP, TData>
where
	TData: ConstByteBufData,
{
	#[inline]
	fn eq(&self, other: &[u8]) -> bool {
		PartialEq::eq(self.as_bytes(), other)
	}
}

impl<const CAP: usize, TData> PartialEq<&'_ [u8]> for ConstByteBuf<CAP, TData>
where
	TData: ConstByteBufData,
{
	#[inline]
	fn eq(&self, other: &&[u8]) -> bool {
		PartialEq::eq(self.as_bytes(), *other)
	}
}

impl<const CAP: usize, TData> PartialEq<&'_ mut [u8]> for ConstByteBuf<CAP, TData>
where
	TData: ConstByteBufData,
{
	#[inline]
	fn eq(&self, other: &&mut [u8]) -> bool {
		PartialEq::eq(self.as_bytes(), *other)
	}
}

impl<const CAP: usize, TData> Hash for ConstByteBuf<CAP, TData>
where
	TData: ConstByteBufData,
{
	#[inline]
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		Hash::hash(self.as_bytes(), state)
	}
}

/// Error type indicating buffer overflow during write.
///
/// Returned when a write operation exceeds the fixed capacity of a `ConstByteBuf`.
#[repr(transparent)]
pub struct StackOverflow;
