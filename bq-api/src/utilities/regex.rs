pub(super) const USERNAME_REGEX: &str = r"^[a-zA-Z0-9\-\_\.]{1,25}$";
pub(super) const NAME_REGEX: &str = r"^[a-zA-Z]+(\s[a-zA-Z]+)*$";
pub(super) const EMAIL_REGEX: &str = r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,4}$";
pub(super) const PASSWORD_REGEX: &str = r"^.{8,200}$";
pub(super) const COMMENT_REGEX: &str = r#"^[a-zA-Z0-9\s.,!?'"()-]{0,150}$"#;
pub(super) const ENTRY_TITLE_REGEX: &str = r"^[a-zA-Z0-9+-/()]+(\s[a-zA-Z0-9+-/()]+)*$";