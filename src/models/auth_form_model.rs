use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;
use validator::Validate;

static EMAIL_RX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap());

#[derive(Deserialize, Validate)]
pub struct AuthFormModel {
    #[validate(regex(path=*EMAIL_RX, message="Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
