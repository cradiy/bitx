use crate::Bit;
mod impl_bitx;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitx<const N: usize>([Bit; N]);

pub type Bit8 = Bitx<8>;
pub type Bit16 = Bitx<16>;
pub type Bit24 = Bitx<24>;
pub type Bit32 = Bitx<32>;
pub type Bit64 = Bitx<64>;

#[cfg(feature = "bit128")]
pub type Bit128 = Bitx<128>;

use core::ops::{Bound, RangeBounds};

macro_rules! impl_dc {
    ($($n:expr), +) => {
        $(
            impl Default for Bitx<$n> {
                fn default() -> Self {
                    Self(Default::default())
                }
            }
            #[cfg(feature = "copy")]
            impl Copy for Bitx<$n> {}
        )+
    };
}
impl_dc! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,27,28, 29, 30, 31, 32
}

macro_rules! contruct {

    ($n:expr, $value:expr) => {{
        contruct!(@bytes $n, $value.to_le_bytes())
    }};
    (@bytes $n:expr, $bytes:expr) =>  {
        {
            if $bytes.len() != ($n / 8) {
                panic!("The `bytes` len must be equal to {}.", $n / 8)
            }
            let mut bitx = [Bit::default(); $n];
            for i in 0..$n / 8 {
                let j = i * 8;
                bitx[j..j + 8].copy_from_slice(&Bit::from_u8($bytes[i]));
            }
            bitx
        }
    }
}
#[inline]
fn compute<const N: usize>(slice: &[Bit]) -> [u8; N] {
    let mut bytes = [0; N];
    for i in 0..N {
        let j = i * 8;
        bytes[i] = _compute(&slice[j..j + 8]);
    }
    bytes
}

#[inline]
fn _compute(slice: &[Bit]) -> u8 {
    slice
        .iter()
        .enumerate()
        .fold(0, |output, (idx, bit)| output + (bit.value() << idx))
}

macro_rules! fn_impl {
    ($(($n:expr, $t:ident)), +) => {
        $(
            impl Bitx<$n> {
                pub fn new(value: $t) -> Self {
                    Self(contruct!($n, value))
                }
                pub fn value(&self) ->$t {
                    $t::from_le_bytes(compute(&self.0))
                }
            }
        )+
    };
}

fn_impl! {
    (8, u8),
    (16, u16),
    (32, u32),
    (64, u64)
}

#[cfg(feature = "bit128")]
fn_impl!((128, u128));

impl Bit24 {
    /// Creates a new [`Bit24`].
    pub fn new(value: u32) -> Self {
        let bytes = value.to_le_bytes();
        Self(contruct!(@bytes 24, &bytes[..3]))
    }
    pub fn value(&self) -> u32 {
        let bytes: [u8; 3] = compute(&self.0);
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0])
    }
}

impl<const N: usize> Bitx<N> {
    pub fn from_bits(bits: [Bit; N]) -> Self {
        Self(bits)
    }

    /// Read any `1` ~ `8` bits and convert to `u8`
    ///
    /// # Panic
    /// The function will panic if more than 8 bits are read.
    ///
    /// # Example
    /// ```rust
    /// use bitx::Bitx;
    ///
    /// let bit8 = Bitx::<8>::new(0b101);
    /// // 0b101
    /// assert_eq!(bit8.read_bits(0..3), 5);
    /// // 0b10
    /// assert_eq!(bit8.read_bits(1..3), 2);
    /// // 0b0
    /// assert_eq!(bit8.read_bits(1..2), 0);
    ///
    /// ```
    ///
    pub fn read_bits(&self, range: impl RangeBounds<usize>) -> u8 {
        _compute(&self[_range(range)])
    }

    fn from_bits_(bits: &[Bit]) -> Self {
        todo!()
    }

}

fn _range(range: impl RangeBounds<usize>) -> core::ops::RangeInclusive<usize> {
    let start = match range.start_bound() {
        Bound::Included(i) => *i,
        Bound::Excluded(e) => *e,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(i) => *i,
        Bound::Excluded(e) => {
            if *e == 0 {
                0
            } else {
                *e - 1
            }
        }
        Bound::Unbounded => 7,
    };
    if end - start > 8 {
        panic!("More than 8 bits are read.")
    }
    start..=end
}

pub struct BitxReader<const N: usize> {
    inner: [Bit; N],
    index: usize,
}

impl<const N: usize> BitxReader<N> {
    #[must_use]
    pub fn read_h4bits(&mut self, buf: &mut u8) -> usize {
        let len = self.remain().min(4);
        
        todo!()
    }
    // #[must_use]
    // pub fn read_l4bits(&mut self, buf: &mut u8) -> usize {}
    pub fn remain(&self) -> usize {
        N - self.index
    }
}
