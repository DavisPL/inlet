use std::fmt::{Debug, Display};

use crate::ast::path::Path;

#[derive(Clone)]
pub enum Origin {
    Universal,
    Exact(Path),
}

impl Origin {
    pub fn satisfies(&self, target: &Origin) -> bool {
        match self {
            Self::Universal => target == &Origin::Universal,
            Self::Exact(p_self) => match target {
                Origin::Universal => true,
                Origin::Exact(p_target) => p_self
                    .to_string()
                    .as_str()
                    .starts_with(p_target.to_string().as_str()),
            },
        }
    }
}

impl Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Universal => write!(f, "*"),
            Self::Exact(path) => write!(f, "{}", path),
        }
    }
}

impl Debug for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Universal => write!(f, "*"),
            Self::Exact(path) => write!(f, "{}", path),
        }
    }
}

impl PartialEq for Origin {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Origin::Universal => match other {
                Origin::Universal => true,
                Origin::Exact(_) => false,
            },
            Origin::Exact(p1) => match other {
                Origin::Universal => false,
                Origin::Exact(p2) => p1.to_string() == p2.to_string(),
            },
        }
    }
}
