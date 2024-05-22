use super::regex::*;
use anyhow::anyhow;
use regex::Regex;
use serde::{Deserialize, Serialize};

macro_rules! parsable {
    ($t:ident, $regex:expr) => {
        #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
        pub struct $t(String);

        impl From<$t> for String {
            fn from(item: $t) -> Self {
                item.0
            }
        }
        
        impl From<String> for $t {
            fn from(s: String) -> Self {
                $t(s)
            }
        }

        impl $t {
            pub fn parse(value: String) -> anyhow::Result<Self> {
                let pattern = Regex::new($regex)?;

                match pattern.is_match(&value) {
                    true => Ok(Self(value)),
                    false => Err(anyhow!("Invalid value"))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

parsable!(Username, USERNAME_REGEX);
parsable!(Name, NAME_REGEX);
parsable!(Email, EMAIL_REGEX);
parsable!(PlainTextPassword, PASSWORD_REGEX);
parsable!(Comment, COMMENT_REGEX);
parsable!(SubtaskTitle, ENTRY_TITLE_REGEX);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_username_valid() {
        let username = Username::parse("john_doe".to_string()).unwrap();
        assert_eq!(username.as_str(), "john_doe");
    }

    #[test]
    fn test_parse_username_invalid() {
        let result = Username::parse("john doe".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_name_valid() {
        let name = Name::parse("John Doe".to_string()).unwrap();
        assert_eq!(name.as_str(), "John Doe");
    }

    #[test]
    fn test_parse_name_invalid() {
        let result = Name::parse("John123".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_email_valid() {
        let email = Email::parse("john@example.com".to_string()).unwrap();
        assert_eq!(email.as_str(), "john@example.com");
    }

    #[test]
    fn test_parse_email_invalid() {
        let result = Email::parse("john@example".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_password_valid() {
        let password = PlainTextPassword::parse("P@ssw0rd".to_string()).unwrap();
        assert_eq!(password.as_str(), "P@ssw0rd");
    }

    #[test]
    fn test_parse_password_invalid() {
        let result = PlainTextPassword::parse("123".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_comment_valid() {
        let comment = Comment::parse("This is a comment".to_string()).unwrap();
        assert_eq!(comment.as_str(), "This is a comment");
    }

    #[test]
    fn test_parse_comment_invalid() {
        let result = Comment::parse("<script></script>".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_entry_title_valid() {
        let entry_title = SubtaskTitle::parse("My Entry".to_string()).unwrap();
        assert_eq!(entry_title.as_str(), "My Entry");
    }

    #[test]
    fn test_parse_entry_title_invalid() {
        let result = SubtaskTitle::parse("".to_string());
        assert!(result.is_err());
    }
}