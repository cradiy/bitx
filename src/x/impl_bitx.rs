use core::ops::RangeFull;

#[cfg(feature = "bit128")]
use super::Bit128;
use super::Bit16;
use super::Bit24;
use super::Bit32;
use super::Bit64;
use super::Bitx;
use super::Bit8;
use crate::Bit;

impl<const N: usize> PartialOrd for Bitx<N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Bitx<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let b1 = &self.0;
        let b2 = &other.0;
        for i in (0..8).rev() {
            if b1[i] == b2[2] {
                continue;
            } else if *b1[i] {
                return core::cmp::Ordering::Greater;
            } else if *b2[i] {
                return core::cmp::Ordering::Less;
            }
        }
        core::cmp::Ordering::Equal
    }
}

unsafe impl<const N: usize> Sync for Bitx<N> {}
unsafe impl<const N: usize> Send for Bitx<N> {}
impl<const N: usize> Unpin for Bitx<N> {}

macro_rules! impl_fmt {
    (@fmt $n:expr, $output:ident, $($_trait:ident), +) => {
        $(
            impl core::fmt::$_trait for Bitx<$n> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    core::fmt::$_trait::fmt(&impl_fmt!(@compute $output, self), f)
                }
            }
        ) +
    };
    (@compute $out:ident, $bit:expr) => {
        $bit.0.iter().enumerate().fold(0, |output: $out, (idx, bit)| {
            output + (if **bit { 1 } else { 0 } << idx)
        })
    };
    ($ty:ident, $($n:expr), +) => {
        $(
            impl_fmt!(@fmt $n, $ty, Display, Binary, LowerExp, LowerHex, Octal );
        ) +
    };
}

impl_fmt!(u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_fmt!(u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_fmt!(u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
impl_fmt!(
    u64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
    55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

#[cfg(feature = "bit128")]
impl_fmt!(
    u128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
    87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
    108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
    127, 128
);
macro_rules! impl_from {
    ($(($struct:ident, $ty:ident)), +) => {
        $(
            impl From<$struct> for $ty {
                fn from(value: $struct) -> Self {
                    value.value()
                }
            }
            impl From<$ty> for $struct {
                fn from(value: $ty) -> Self {
                    $struct::new(value)
                }
            }

        ) +
    };
}
impl_from! {
    (Bit8, u8),
    (Bit16, u16),
    (Bit24, u32),
    (Bit32, u32),
    (Bit64, u64)
}

#[cfg(feature = "bit128")]
impl_from!((Bit128, u128));


impl<const N: usize> core::ops::Index<usize> for Bitx<N> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
macro_rules! impl_range {
    ($($range:ident), +) => {
        $(
            impl<const N: usize> core::ops::Index<core::ops::$range<usize>> for Bitx<N> {
                type Output = [Bit];

                fn index(&self, index: core::ops::$range<usize>) -> &Self::Output {
                    &self.0[index]
                }
            }
            impl<const N: usize> core::ops::IndexMut<core::ops::$range<usize>> for Bitx<N> {
                fn index_mut(&mut self, index: core::ops::$range<usize>) -> &mut Self::Output {
                    &mut self.0[index]
                }
            }
        )+
    };
}
impl_range! {
    Range, RangeFrom, RangeTo, RangeToInclusive, RangeInclusive
}

impl<const N: usize> core::ops::IndexMut<usize> for Bitx<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut *self.0[index]
    }
}
impl<const N: usize> core::ops::Index<RangeFull> for Bitx<N> {
    type Output = [Bit];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}
impl<const N: usize> core::ops::IndexMut<RangeFull> for Bitx<N> {
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> IntoIterator for Bitx<N> {
    type Item = Bit;

    type IntoIter = core::array::IntoIter<Bit, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
