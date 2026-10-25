#[derive(PartialEq)]
pub enum Sex {
    Male,
    Female,
}

impl From<&str> for Sex {
    fn from(s: &str) -> Self {
        match s {
            "Masculin" | "♂️" => Self::Male,
            "Féminin" | "♀️" => Self::Female,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sex::Male => "♂️".to_string(),
                Sex::Female => "♀️".to_string(),
            }
        )
    }
}
