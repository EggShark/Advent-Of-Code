use std::fmt::{Display, Formatter, Result};

pub enum SolutionType {
    I32(i32),
    I64(i64),
    I128(i128),
    U32(u32),
    U64(u64),
    U128(u128),
    Str(String),
}

pub type SolutionPair = (SolutionType, SolutionType);

pub struct Solution {
    pub solution: SolutionPair,
    pub time_ms: f64,
}

impl Display for SolutionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::I32(x) => x.fmt(f),
            Self::I64(x) => x.fmt(f),
            Self::I128(x) => x.fmt(f),
            Self::U32(x) => x.fmt(f),
            Self::U64(x) => x.fmt(f),
            Self::U128(x) => x.fmt(f),
            Self::Str(x) => x.fmt(f),
        }
    }
}