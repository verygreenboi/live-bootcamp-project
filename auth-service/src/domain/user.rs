use crate::domain::email::Email;
use crate::domain::password::Password;

#[derive(Clone)]
pub struct User {
    pub(crate) email: Email,
    pub(crate) password: Password,
    requires_2fa: bool
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        let email = Email::parse(email.as_str()).expect("Invalid email");
        let password = Password::parse(password.as_str()).expect("Invalid password");
        Self { email, password, requires_2fa }
    }
}