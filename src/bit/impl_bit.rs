use super::Bit;

mod impl_ops {
    use super::Bit;
    use core::ops::*;

    impl Not for Bit {
        type Output = bool;

        fn not(self) -> Self::Output {
            !self.0
        }
    }
    macro_rules! impl_ops_ {
        ($(($_trait:ident, $fn:ident)), +) => {
            $(
                impl $_trait for Bit {
                    type Output = Bit;

                    fn $fn(self, rhs: Self) -> Self::Output {
                        Bit(self.0.$fn(rhs.0))
                    }
                }
            )+
        };
    }
    impl_ops_! {
        (BitOr, bitor),
        (BitAnd, bitand),
        (BitXor, bitxor)
    }
    impl BitAndAssign for Bit {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0.bitand_assign(rhs.0)
        }
    }
    impl BitOrAssign for Bit {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0.bitor_assign(rhs.0)
        }
    }
    impl BitXorAssign for Bit {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.0.bitxor_assign(rhs.0)
        }
    }
    impl Deref for Bit {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for Bit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

mod impl_fmt {
    use super::Bit;
    impl core::fmt::Display for Bit {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(if self.0 { "1" } else { "0" })
        }
    }
}
impl From<Bit> for u8 {
    fn from(value: Bit) -> Self {
        value.0 as u8
    }
}
impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        value.0
    }
}
unsafe impl Sync for Bit {}
unsafe impl Send for Bit {}
impl Unpin for Bit {}
