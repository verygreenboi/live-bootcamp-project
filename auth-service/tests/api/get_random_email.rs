const EMAIL_PREFIX: &str = "test";
const EMAIL_DOMAIN: &str = "example.com";

pub fn get_random_email() -> String {
    format!("{}{}@{}", EMAIL_PREFIX, uuid::Uuid::new_v4(), EMAIL_DOMAIN)

}