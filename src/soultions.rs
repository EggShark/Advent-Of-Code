use std::fmt::{Display, Formatter, Result};

pub enum Solution {
    I32(i32),
    I64(i64),
    I128(i128),
    U32(u32),
    U64(u64),
    U128(u128),
    Str(String),
}

pub type SolutionPair = (Solution, Solution);

impl Display for Solution {
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