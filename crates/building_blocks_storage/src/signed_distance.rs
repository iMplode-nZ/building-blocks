use serde::{Deserialize, Serialize};

pub trait SignedDistance: Into<f32> {
    fn is_negative(self) -> bool;
}

impl SignedDistance for f32 {
    #[inline]
    fn is_negative(self) -> bool {
        self < 0.0
    }
}

/// A signed distance value in the range `[-1.0, 1.0]` with 8 bits of precision.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Sd8(pub i8);
/// A signed distance value in the range `[-1.0, 1.0]` with 16 bits of precision.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Sd16(pub i16);

impl Sd8 {
    pub const RESOLUTION: f32 = std::i8::MAX as f32;
    pub const PRECISION: f32 = 1.0 / Self::RESOLUTION;
    pub const NEG_ONE: Self = Self(std::i8::MIN);
    pub const ONE: Self = Self(std::i8::MAX);
}

impl Sd16 {
    pub const RESOLUTION: f32 = std::i16::MAX as f32;
    pub const PRECISION: f32 = 1.0 / Self::RESOLUTION;
    pub const NEG_ONE: Self = Self(std::i16::MIN);
    pub const ONE: Self = Self(std::i16::MAX);
}

impl Default for Sd8 {
    fn default() -> Self {
        Self::ONE
    }
}
impl From<Sd8> for f32 {
    fn from(s: Sd8) -> f32 {
        s.0 as f32 * Sd8::PRECISION
    }
}
impl From<f32> for Sd8 {
    fn from(s: f32) -> Self {
        Sd8((Self::RESOLUTION * s.min(1.0).max(-1.0)) as i8)
    }
}
impl SignedDistance for Sd8 {
    #[inline]
    fn is_negative(self) -> bool {
        self.0 < 0
    }
}

impl Default for Sd16 {
    fn default() -> Self {
        Self::ONE
    }
}
impl From<Sd16> for f32 {
    fn from(s: Sd16) -> f32 {
        s.0 as f32 * Sd16::PRECISION
    }
}
impl From<f32> for Sd16 {
    fn from(s: f32) -> Self {
        Sd16((Self::RESOLUTION * s.min(1.0).max(-1.0)) as i16)
    }
}
impl SignedDistance for Sd16 {
    #[inline]
    fn is_negative(self) -> bool {
        self.0 < 0
    }
}
