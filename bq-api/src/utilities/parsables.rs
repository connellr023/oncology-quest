use super::regex::*;
use anyhow::anyhow;
use regex::Regex;
use serde::{Serialize, Deserialize};

pub trait Parsable: for<'de> Deserialize<'de> + Sized {
    /// Parses a string into a value of this type.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The string to parse.
    /// 
    /// # Returns
    /// 
    /// Returns an error if the string is not valid.
    fn parse(value: String) -> anyhow::Result<Self>;

    /// Returns the parsed value as a string slice.
    fn as_str(&self) -> &str;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Username(String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Name(String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Email(String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlainTextPassword(String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Comment(String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EntryTitle(String);

impl Parsable for Username {
    fn parse(value: String) -> anyhow::Result<Self> {
        let pattern = Regex::new(USERNAME_REGEX)?;

        match pattern.is_match(&value) {
            true => Ok(Username(value)),
            false => Err(anyhow!("Invalid username"))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parsable for Name {
    fn parse(value: String) -> anyhow::Result<Self> {
        let pattern = Regex::new(NAME_REGEX)?;

        match pattern.is_match(&value) {
            true => Ok(Name(value)),
            false => Err(anyhow!("Invalid name"))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parsable for Email {
    fn parse(value: String) -> anyhow::Result<Self> {
        let pattern = Regex::new(EMAIL_REGEX)?;

        match pattern.is_match(&value) {
            true => Ok(Email(value)),
            false => Err(anyhow!("Invalid email"))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parsable for PlainTextPassword {
    fn parse(value: String) -> anyhow::Result<Self> {
        let pattern = Regex::new(PASSWORD_REGEX)?;

        match pattern.is_match(&value) {
            true => Ok(PlainTextPassword(value)),
            false => Err(anyhow!("Invalid password"))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parsable for Comment {
    fn parse(value: String) -> anyhow::Result<Self> {
        let pattern = Regex::new(COMMENT_REGEX)?;

        match pattern.is_match(&value) {
            true => Ok(Comment(value)),
            false => Err(anyhow!("Invalid comment"))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parsable for EntryTitle {
    fn parse(value: String) -> anyhow::Result<Self> {
        let pattern = Regex::new(ENTRY_TITLE_REGEX)?;

        match pattern.is_match(&value) {
            true => Ok(EntryTitle(value)),
            false => Err(anyhow!("Invalid entry title"))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

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
        assert_eq!(result.unwrap_err().to_string(), "Invalid username");
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
        assert_eq!(result.unwrap_err().to_string(), "Invalid name");
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
        assert_eq!(result.unwrap_err().to_string(), "Invalid email");
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
        assert_eq!(result.unwrap_err().to_string(), "Invalid password");
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
        assert_eq!(result.unwrap_err().to_string(), "Invalid comment");
    }

    #[test]
    fn test_parse_entry_title_valid() {
        let entry_title = EntryTitle::parse("My Entry".to_string()).unwrap();
        assert_eq!(entry_title.as_str(), "My Entry");
    }

    #[test]
    fn test_parse_entry_title_invalid() {
        let result = EntryTitle::parse("".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid entry title");
    }
}