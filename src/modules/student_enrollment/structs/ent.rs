#[derive(serde::Deserialize)]
pub struct Oauth2Token {
    pub access_token: String,
}

#[derive(serde::Deserialize)]
pub struct ApiUser {
    pub id: String,
    pub login: String,
    #[serde(default, rename = "activationCode")]
    pub activation_code: Option<String>,
}
