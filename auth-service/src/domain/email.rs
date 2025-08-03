#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        match s.contains("@") {
            true => Ok(Self(String::from(s))),
            false => Err("missing '@' sign in email address".to_string()),
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]

mod tests {
    use super::Email;

    #[test]
    fn is_should_return_the_email_address_when_valid() {
        let expected = Ok(Email(String::from("foo@example.com")));
        let actual = Email::parse(String::from("foo@example.com"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn is_should_return_an_error_when_email_address_does_not_contain_at() {
        let actual = Email::parse(String::from("foo")).is_err();
        assert_eq!(actual, true);
    }

    #[test]
    fn is_should_return_the_email_address_when_as_ref_is_called() {
        let expected = String::from("foo@example.com");
        let email = Email::parse(expected.clone()).unwrap();

        let actual = email.as_ref();

        assert_eq!(actual, &expected);
    }
}
