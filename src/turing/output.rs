use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TuringOutput {
    Undefined(usize),
    Defined((usize, u32)),
}

impl Default for TuringOutput {
    fn default() -> Self {
        Self::Undefined(0)
    }
}

impl Display for TuringOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined(_) => write!(f, "Undefined"),
            Self::Defined((pos, val)) => write!(f, "Defined({}, {})", pos, val),
        }
    }
}
