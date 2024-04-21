use lazy_static::lazy_static;

pub struct Settings {
    pub auth_cookie_name: String,
    pub cookie_secret: String,
    pub implicit_assertion: String,

    pub email_host: String,     // For gmail it's smtp.gmail.com
    pub email_user: String,     // For gmail it's your email
    pub email_password: String, // For gmail it's an "app password"
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        Settings {
            auth_cookie_name: "auth".to_string(),
            cookie_secret: "k4.local.HlMBf5cHedQSsyRtvxl2ACFTt7hGaOi7vLZxmZzv6TY".to_string(),
            implicit_assertion: "abc".to_string(),
            email_host: "smtp.gmail.com".to_string(),
            email_user: std::env::var("APP_USER").unwrap(),
            email_password: std::env::var("APP_PASS").unwrap(),
        }
    };
}

pub fn get_settings() -> &'static Settings {
    &SETTINGS
}
