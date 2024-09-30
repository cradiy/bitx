use crate::Bit;


pub struct BitVec<const MAX: usize> {
    inner: BitV,
    len: usize,
}

impl<const MAX: usize> BitVec<MAX> {
    pub fn new() -> Self {
        Self {
            inner: BitV::Bit8(Default::default()),
            len: 0,
        }
    }
    pub fn cap(&self) -> usize {
        self.inner.cap()
    }
    pub fn push<B: Into<Bit>>(&mut self, val: B) -> bool {
        if self.len >= 127 {
            return false;
        } else if self.len == self.cap() {
            self.inner.scaling();
        }
        self.inner.set(self.len, val.into());
        true
    }
}
#[derive(Clone)]
enum BitV {
    Bit8([Bit; 8]),
    Bit16([Bit; 16]),
    Bit32([Bit; 32]),
    Bit64([Bit; 64]),
    Bit128([Bit; 128]),
}

impl BitV {
    // fn new_with_capaity(cap: usize) 
    fn cap(&self) -> usize {
        match self {
            BitV::Bit8(_) => 8,
            BitV::Bit16(_) => 16,
            BitV::Bit32(_) => 32,
            BitV::Bit64(_) => 64,
            BitV::Bit128(_) => 128,
        }
    }
    fn value_mut(&mut self) -> &mut [Bit] {
        match self {
            BitV::Bit8(inner) => inner,
            BitV::Bit16(inner) => inner,
            BitV::Bit32(inner) => inner,
            BitV::Bit64(inner) => inner,
            BitV::Bit128(inner) => inner,
        }
    }
    fn set(&mut self, index: usize, value: impl Into<Bit>) {
        self.value_mut()[index] = value.into();
    }
    fn scaling(&mut self) -> bool {
        *self = match self {
            BitV::Bit8(inner) => {
                let mut bits = [Bit::default(); 16];
                bits[0..8].copy_from_slice(inner);
                Self::Bit16(bits)
            }
            BitV::Bit16(inner) => {
                let mut bits = [Bit::default(); 32];
                bits[0..16].copy_from_slice(inner);
                Self::Bit32(bits)
            }
            BitV::Bit32(inner) => {
                let mut bits = [Bit::default(); 64];
                bits[0..32].copy_from_slice(inner);
                Self::Bit64(bits)
            }
            BitV::Bit64(inner) => {
                let mut bits = [Bit::default(); 128];
                bits[0..64].copy_from_slice(inner);
                Self::Bit128(bits)
            }
            _ => return  false,
        };
        true
    }
}
