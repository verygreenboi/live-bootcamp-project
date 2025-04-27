use crate::domain::email::Email;

#[derive(Clone)]
pub struct User {
    pub(crate) email: Email,
    pub(crate) password: String,
    requires_2fa: bool
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        let email = Email::parse(email.as_str()).unwrap();
        Self { email, password, requires_2fa }
    }
}