#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Password, String> {
        match s.len() >= 8 {
            true => Ok(Self(String::from(s))),
            false => Err("length must be greter than or equal to 8".to_string()),
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]

mod tests {
    use super::Password;

    #[test]
    fn is_should_return_the_password_when_len_gte_8() {
        let expected = Ok(Password(String::from("12345678")));
        let actual = Password::parse(String::from("12345678"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn is_should_return_an_error_when_password_is_less_than_8_chars() {
        let actual = Password::parse(String::from("1234567")).is_err();
        assert_eq!(actual, true);
    }
}
