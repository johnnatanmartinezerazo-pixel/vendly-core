use regex::Regex;
use std::sync::LazyLock;

pub struct ValidationRule {
    pub name: &'static str,
    pub regex: Regex,
}

pub static EMAIL_REGEX: LazyLock<ValidationRule> = LazyLock::new(|| ValidationRule {
    name: "EMAIL_REGEX",
    regex: Regex::new(
        r"^[A-Za-z0-9](?:[A-Za-z0-9._-]{2,63}[A-Za-z0-9])?@[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?(?:\.[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?)*\.[A-Za-z]{2,24}$"
    ).expect("Invalid EMAIL_REGEX"),
});

pub static ROLE_NAME_REGEX: LazyLock<ValidationRule> = LazyLock::new(|| ValidationRule {
    name: "ROLE_NAME_REGEX",
    regex: Regex::new(r"^[a-z][a-z0-9_-]{2,49}$").expect("Invalid ROLE_NAME_REGEX"),
});

pub static USERNAME_REGEX: LazyLock<ValidationRule> = LazyLock::new(|| ValidationRule {
    name: "USERNAME_REGEX",
    regex: Regex::new(r"^(?=.{6,30}$)[a-z][a-z0-9](?:[._-]?[a-z0-9])*$").expect("Invalid USERNAME_REGEX"),
});
