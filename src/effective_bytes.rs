// fixed_str/src/effective_bytes.rs

use super::*;

/// A trait for extracting the effective bytes from a value.
/// The effective bytes are defined as all bytes up until the first null byte (`\0`).
pub trait EffectiveBytes {
    /// Returns the effective bytes up until the first null byte.
    fn effective_bytes(&self) -> &[u8];
}

//******************************************************************************
//  Implementations
//******************************************************************************

impl<const N: usize> EffectiveBytes for FixedStr<N> {
    fn effective_bytes(&self) -> &[u8] {
        &self[..self.len()]
    }
}

impl<const N: usize> EffectiveBytes for &FixedStr<N> {
    fn effective_bytes(&self) -> &[u8] {
        (*self).effective_bytes()
    }
}

impl EffectiveBytes for [u8] {
    fn effective_bytes(&self) -> &[u8] {
        let end = find_first_null(self);
        &self[..end]
    }
}

impl<const N: usize> EffectiveBytes for [u8; N] {
    fn effective_bytes(&self) -> &[u8] {
        let end = find_first_null(self);
        &self[..end]
    }
}

impl EffectiveBytes for &str {
    fn effective_bytes(&self) -> &[u8] {
        self.as_bytes().effective_bytes()
    }
}

#[cfg(feature = "std")]
impl EffectiveBytes for String {
    fn effective_bytes(&self) -> &[u8] {
        self.as_bytes().effective_bytes()
    }
}

//******************************************************************************
//  Iterator
//******************************************************************************

/// An iterator over the effective bytes of a fixed‑capacity string,
/// stopping at the first null byte.
pub struct EffectiveBytesIter<const N: usize> {
    pub(super) data: [u8; N],
    pub(super) index: usize,
    pub(super) len: usize,
}

impl<const N: usize> Iterator for EffectiveBytesIter<N> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let byte = self.data[self.index];
            self.index += 1;
            Some(byte)
        } else {
            None
        }
    }
}

//******************************************************************************
//  Tests
//******************************************************************************

#[cfg(test)]
mod effbyte_tests {
    use super::*;

    #[test]
    fn test_effective_bytes_trait() {
        let fixed = FixedStr::<10>::new("Hi");
        let bytes = fixed.effective_bytes();
        assert_eq!(bytes, b"Hi");

        // For a byte slice with an embedded null, effective_bytes stops at the first zero.
        let slice: &[u8] = b"abc\0def";
        let effective = slice.effective_bytes();
        assert_eq!(effective, b"abc");

        // For &str, effective_bytes should simply return the full UTF‑8 bytes.
        let s = "hello";
        let effective_str = s.effective_bytes();
        assert_eq!(effective_str, b"hello");
    }
}
