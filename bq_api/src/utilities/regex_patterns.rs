pub const USERNAME_REGEX: &str = r"^[a-zA-Z0-9\-\_\.]{0,25}$";
pub const NAME_REGEX: &str = r"^[a-zA-Z]+(\s[a-zA-Z]+)*$";
pub const EMAIL_REGEX: &str = r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,4}$";
pub const PASSWORD_REGEX: &str = r"^.{8,200}$";