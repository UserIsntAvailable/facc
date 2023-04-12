#![no_std]
#![deny(rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

const fn fourcc_impl<const IS_LITTLE: bool>(four: &[u8]) -> u32 {
    assert!(four.len() == 4, "FourCC must be 4 ASCII characters");

    let mut result = 0;
    let mut remaining = four;

    while let [head @ .., current] = remaining {
        assert!(current.is_ascii(), "FourCC must only have ASCII characters");

        let current = *current as u32;
        result = if IS_LITTLE {
            (result >> 8) | (current << 24)
        } else {
            (result << 8) | current
        };

        remaining = head;
    }

    result
}

#[inline]
pub const fn fourcc(four: &[u8]) -> u32 {
    fourcc_impl::<{ cfg!(target_endian = "little") }>(four)
}

#[inline]
pub const fn fourcc_le(four: &[u8]) -> u32 {
    fourcc_impl::<true>(four)
}

#[inline]
pub const fn fourcc_be(four: &[u8]) -> u32 {
    fourcc_impl::<false>(four)
}

#[inline]
pub fn is_fourcc<Bytes>(four: Bytes) -> bool
where
    Bytes: AsRef<[u8]>,
{
    let bytes = four.as_ref();
    bytes.len() == 4 && bytes.iter().all(u8::is_ascii)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_works() {
        const _: () = assert!(fourcc_le(b"avc1") == 0x61766331);
    }

    #[test]
    fn be_works() {
        const _: () = assert!(fourcc_be(b"avc1") == 0x31637661);
    }

    #[test]
    fn is_works() {
        assert!(is_fourcc("avc1"))
    }

    #[test]
    #[should_panic(expected = "FourCC must be 4 ASCII characters")]
    fn str_is_not_4_len() {
        fourcc(b"avc");
    }

    #[test]
    #[should_panic(expected = "FourCC must only have ASCII characters")]
    fn str_constains_non_ascii_chars() {
        fourcc("神 ".as_bytes());
    }
}
