use super::regex::*;
use anyhow::anyhow;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};

trait Deserializable: for<'de> Deserialize<'de> + Sized {
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>;
}

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

macro_rules! parsable {
    ($t:ident, $regex:expr) => {
        #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
        pub struct $t(String);

        impl Deserializable for $t {
            fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
            where D: Deserializer<'de> {
                let value = String::deserialize(deserializer)?;
                match Self::parse(value) {
                    Ok(parsed) => Ok(parsed),
                    Err(err) => Err(serde::de::Error::custom(err.to_string()))
                }
            }
        }

        impl Parsable for $t {
            fn parse(value: String) -> anyhow::Result<Self> {
                let pattern = Regex::new($regex)?;

                match pattern.is_match(&value) {
                    true => Ok(Self(value)),
                    false => Err(anyhow!("Invalid value"))
                }
            }

            fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryIndex(Box<[usize]>);

impl Deserializable for EntryIndex {
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let slice = Box::<[usize]>::deserialize(deserializer)?;
        match Self::from_boxed_slice(slice) {
            Ok(entry_index) => Ok(entry_index),
            Err(err) => Err(serde::de::Error::custom(err.to_string()))
        }
    }
}

impl EntryIndex {
    pub fn from_boxed_slice(slice: Box<[usize]>) -> anyhow::Result<Self> {
        if slice.is_empty() || slice.len() > 3 {
            return Err(anyhow!("Invalid index tuple length"));
        }

        Ok(Self(slice))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn supertask_entry_index(&self) -> usize {
        self.0[0]
    }

    pub fn task_entry_index(&self) -> anyhow::Result<usize> {
        if self.len() < 2 {
            return Err(anyhow!("Task entry index not found"));
        }

        Ok(self.0[1])
    }

    pub fn subtask_entry_index(&self) -> anyhow::Result<usize> {
        if self.len() < 3 {
            return Err(anyhow!("Subtask entry index not found"));
        }

        Ok(self.0[2])
    }
}

parsable!(Username, USERNAME_REGEX);
parsable!(Name, NAME_REGEX);
parsable!(Email, EMAIL_REGEX);
parsable!(PlainTextPassword, PASSWORD_REGEX);
parsable!(Comment, COMMENT_REGEX);
parsable!(EntryTitle, ENTRY_TITLE_REGEX);

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
        let entry_title = EntryTitle::parse("My Entry".to_string()).unwrap();
        assert_eq!(entry_title.as_str(), "My Entry");
    }

    #[test]
    fn test_parse_entry_title_invalid() {
        let result = EntryTitle::parse("".to_string());
        assert!(result.is_err());
    }
}