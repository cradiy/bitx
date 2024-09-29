mod impl_bit;

macro_rules! __bits {
    ($byte: expr, $($offset:expr), +) => {
        [ $( Bit::new(($byte & (0x01 << $offset) != 0))), + ]
    };
}


#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Bit(bool);

impl Bit {
    pub fn new(bit: bool) -> Bit {
        Self(bit)
    }
    pub fn from_u8(byte: u8) -> [Bit; 8] {
        __bits!(byte, 0, 1, 2, 3, 4, 5, 6, 7)
    }
    
    pub fn bit(&self) -> bool {
        self.0
    }
    pub fn value(&self) -> u8 {
        self.0 as u8
    }
}

