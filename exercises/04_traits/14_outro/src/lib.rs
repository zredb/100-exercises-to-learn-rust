// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct SaturatingU16(u16);

#[macro_export]
macro_rules! impl_type {
    ($type:ty) => {
        impl From<$type> for SaturatingU16 {
            fn from(value: $type) -> Self {
                SaturatingU16(value as u16)
            }
        }
        impl From<&$type> for SaturatingU16 {
            fn from(value: &$type) -> Self {
                SaturatingU16(*value as u16)
            }
        }
    }
}

impl_type!(u16);
impl_type!(u8);

// impl From<u16> for SaturatingU16 {
//     fn from(value: u16) -> Self {
//         SaturatingU16(value)
//     }
// }
//
// impl From<&u16> for SaturatingU16 {
//     fn from(value: &u16) -> Self {
//         SaturatingU16(*value)
//     }
// }
//
// impl From<u8> for SaturatingU16 {
//     fn from(value: u8) -> Self {
//         SaturatingU16(value as u16)
//     }
// }
// impl From<&u8> for SaturatingU16 {
//     fn from(value: &u8) -> Self {
//         SaturatingU16(*value as u16)
//     }
// }

// impl From<SaturatingU16> for SaturatingU16 {
//     fn from(value: SaturatingU16) -> Self {
//         SaturatingU16(value.0)
//     }
// }



impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16::from(self.0.saturating_add(rhs.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16::from(self.0.saturating_add(rhs))
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16::from(self.0.saturating_add(*rhs))
    }
}

impl PartialEq<SaturatingU16> for SaturatingU16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}