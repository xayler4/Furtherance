#[derive(Clone, PartialEq)]
pub struct FurUser {
    pub email: String,
    pub encrypted_key: String,
    pub key_nonce: String,
    pub access_token: String,
    pub refresh_token: String,
    pub server: String,
}

#[derive(Clone)]
pub struct FurUserFields {
    pub email: String,
    pub password: String,
    pub server: String,
}

impl Default for FurUserFields {
    fn default() -> Self {
        FurUserFields {
            email: String::new(),
            password: String::new(),
            server: String::new(),
        }
    }
}