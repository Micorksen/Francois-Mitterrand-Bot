#[derive(PartialEq)]
pub enum Level {
    Seconde,
    Premiere,
    Terminale,
}

impl From<&str> for Level {
    fn from(s: &str) -> Self {
        match s {
            "Seconde" => Self::Seconde,
            "Première" => Self::Premiere,
            "Terminale" => Self::Terminale,
            _ => unreachable!(),
        }
    }
}
impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Level::Seconde => "Seconde".to_string(),
                Level::Premiere => "Première".to_string(),
                Level::Terminale => "Terminale".to_string(),
            }
        )
    }
}
