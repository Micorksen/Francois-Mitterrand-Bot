pub struct Enrollment {
    pub first_name: String,
    pub last_name: String,
    pub sex: super::Sex,
    pub level: super::Level,
    pub regime: super::Regime,
    pub lv1: Option<String>,
    pub lv2: Option<String>,
    pub specialized_subjects: Option<Vec<String>>,
    pub option: Option<String>,
}
