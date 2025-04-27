#[derive(Debug, PartialEq)]
pub enum PasswordError {
    Empty,
    TooShort(usize),
    MissingUppercase,
    MissingLowercase,
    MissingDigit,
    MissingSpecialChar,
}

impl std::error::Error for PasswordError {}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordError::Empty => write!(f, "password cannot be empty"),
            PasswordError::TooShort(min_length) => write!(f, "password must be at least {} characters long", min_length),
            PasswordError::MissingUppercase => write!(f, "password must contain at least one uppercase letter"),
            PasswordError::MissingLowercase => write!(f, "password must contain at least one lowercase letter"),
            PasswordError::MissingDigit => write!(f, "password must contain at least one digit"),
            PasswordError::MissingSpecialChar => write!(f, "password must contain at least one special character"),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Password, PasswordError> {
        match Self::is_valid(password) {
            Ok(_) => {
                Ok(Password(password.to_string()))
            }
            Err(err) => Err(err),
        }
    }
    
    fn is_valid(password: &str) -> Result<(), PasswordError> {
        // Check if password is empty
        if password.is_empty() {
            return Err(PasswordError::Empty);
        }

        // Check if password is at least 8 characters long
        const MIN_LENGTH: usize = 8;
        if password.len() < MIN_LENGTH {
            return Err(PasswordError::TooShort(MIN_LENGTH)); 
        }

        // Check for at least one uppercase letter
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        if !has_uppercase {
            return Err(PasswordError::MissingUppercase);
        }

        // Check for at least one lowercase letter
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        if !has_lowercase {
            return Err(PasswordError::MissingLowercase);
        }

        // Check for at least one digit
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        if !has_digit {
            return Err(PasswordError::MissingDigit);
        }

        // Check for at least one special character
        let has_special = password.chars().any(|c| !c.is_alphanumeric());
        if !has_special {
            return Err(PasswordError::MissingSpecialChar);
        }
        Ok(())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_password() {
        let valid_password = "Passw0rd!";
        let result = Password::parse(valid_password);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), valid_password);
    }

    #[test]
    fn test_empty_password_error() {
        let empty_password = "";
        let result = Password::parse(empty_password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PasswordError::Empty);
    }

    #[test]
    fn test_short_password_error() {
        let short_password = "Pw1!";
        let result = Password::parse(short_password);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PasswordError::TooShort(8));
    }

    #[test]
    fn test_password_without_uppercase_error() {
        let no_uppercase = "password1!";
        let result = Password::parse(no_uppercase);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PasswordError::MissingUppercase);
    }

    #[test]
    fn test_password_without_lowercase_error() {
        let no_lowercase = "PASSWORD1!";
        let result = Password::parse(no_lowercase);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PasswordError::MissingLowercase);
    }

    #[test]
    fn test_password_without_digit_error() {
        let no_digit = "Password!";
        let result = Password::parse(no_digit);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PasswordError::MissingDigit);
    }

    #[test]
    fn test_password_without_special_char_error() {
        let no_special = "Password1";
        let result = Password::parse(no_special);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PasswordError::MissingSpecialChar);
    }

    #[test]
    fn test_multiple_valid_passwords() {
        let valid_passwords = [
            "Passw0rd!",
            "SecureP@ss123",
            "C0mpl3x_P@ssw0rd",
            "Strong#P4ssword",
            "P@$$w0rd123"
        ];

        for password in valid_passwords.iter() {
            assert!(Password::parse(password).is_ok(), "Password '{}' should be valid", password);
        }
    }

    #[test]
    fn test_error_display_messages() {
        assert_eq!(PasswordError::Empty.to_string(), "password cannot be empty");
        assert_eq!(PasswordError::TooShort(8).to_string(), "password must be at least 8 characters long");
        assert_eq!(PasswordError::MissingUppercase.to_string(), "password must contain at least one uppercase letter");
        assert_eq!(PasswordError::MissingLowercase.to_string(), "password must contain at least one lowercase letter");
        assert_eq!(PasswordError::MissingDigit.to_string(), "password must contain at least one digit");
        assert_eq!(PasswordError::MissingSpecialChar.to_string(), "password must contain at least one special character");
    }
}

