#[derive(PartialEq)]
pub enum Regime {
    Intern,
    Extern,
}

impl From<&str> for Regime {
    fn from(s: &str) -> Self {
        match s {
            "Interne" => Self::Intern,
            "Externe" => Self::Extern,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Regime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Regime::Intern => "Interne".to_string(),
                Regime::Extern => "Externe".to_string(),
            }
        )
    }
}
