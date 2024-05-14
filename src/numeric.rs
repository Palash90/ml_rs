//! This is a set of types that goes hand in hand with tensor module.

pub trait Numeric:
    Copy
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
{
    fn zero() -> Self;
}

impl Numeric for i8 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for i16 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for i32 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for i64 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for i128 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for isize {
    fn zero() -> Self {
        0
    }
}
impl Numeric for u8 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for u16 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for u32 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for u64 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for u128 {
    fn zero() -> Self {
        0
    }
}
impl Numeric for usize {
    fn zero() -> Self {
        0
    }
}
impl Numeric for f32 {
    fn zero() -> Self {
        0.0
    }
}
impl Numeric for f64 {
    fn zero() -> Self {
        0.0
    }
}
