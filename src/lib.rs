use std::mem::size_of;
use std::ops::{BitAnd, BitOrAssign, ShlAssign, Shr};

pub trait Uint<Rhs = Self, Output = Self>:
    ShlAssign<u8> + BitAnd<Rhs, Output = Output> + BitOrAssign<Rhs> + Shr<u8, Output = Output> + Copy
{
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_uint {
    ( $($Ty:ty),+ ) => {
        $(
            impl Uint for $Ty {
                const MAX: Self = <$Ty>::MAX;
                const MIN: Self = <$Ty>::MIN;
            }
        )+
    };
}

impl_uint!(u8, u16, u32, u64, u128);

const fn bit_size<T: Uint>() -> u8 {
    (8 * size_of::<T>()) as u8
}

#[inline]
fn n_bit_mask<T: Uint>(n: u8) -> T {
    T::MAX >> (bit_size::<T>() - n)
}

#[derive(Default)]
pub struct Writer<T: Uint>(T);

impl<T: Uint> Writer<T> {
    pub fn write<B: Into<T>>(mut self, count: u8, src: B) -> Self {
        self.0 <<= count;
        self.0 |= src.into() & n_bit_mask(count);

        self
    }

    pub fn finish(self) -> T {
        self.0
    }
}

pub struct Reader<T: Uint> {
    bit_vec: T,
    pos: u8,
}

impl<T: Uint> Reader<T> {
    #[inline]
    pub fn new(bit_vec: T) -> Self {
        Reader { bit_vec, pos: 0 }
    }

    pub fn read_next(&mut self, count: u8) -> T {
        let shift = bit_size::<T>() - count - self.pos;
        let bits = (self.bit_vec >> shift) & n_bit_mask(count);

        self.pos += count;

        bits
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_read() {
        let bits = Writer::<u128>::default()
            .write(11, 42u32)
            .write(24, 1337u32)
            .write(3, 2u8)
            .write(30, 0u32)
            .write(3, 1u8)
            .write(57, 12u64)
            .finish();

        let mut r = Reader::<u128>::new(bits);

        assert_eq!(r.read_next(11), 42);
        assert_eq!(r.read_next(24), 1337);
        assert_eq!(r.read_next(3), 2);
        assert_eq!(r.read_next(30), 0);
        assert_eq!(r.read_next(3), 1);
        assert_eq!(r.read_next(57), 12);
    }
}
