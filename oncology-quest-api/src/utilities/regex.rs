pub(super) const USERNAME_REGEX: &str = r"^[a-zA-Z0-9\-\_\.]{1,25}$";
pub(super) const NAME_REGEX: &str = r"^[a-zA-Z\s]{1,35}$";
pub(super) const PASSWORD_REGEX: &str = r"^.{8,200}$";
pub(super) const COMMENT_REGEX: &str = r#"^[a-zA-Z0-9\s.,!?'"()-]{0,150}$"#;
pub(super) const ENTRY_TITLE_REGEX: &str = r"^[a-zA-Z0-9+\-/()\s]{1,20}$";