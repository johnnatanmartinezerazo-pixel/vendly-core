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
    ).unwrap(),
});

pub static PHONE_CONTRY_REGEX: LazyLock<ValidationRule> = LazyLock::new(|| ValidationRule {
    name: "COUNTRY_CODE(+XX)",
    regex: Regex::new(r"^\+\d{1,3}[0-9]$").unwrap()
});

pub static PHONE_NUMBER_REGEX: LazyLock<ValidationRule> = LazyLock::new(|| ValidationRule {
    name: "LOCAL_NUMBER(only digits)",
    regex: Regex::new(r"^\d{6,14}[0-9]$").unwrap()
});

pub static LOCALE_REGEX: LazyLock<ValidationRule> = LazyLock::new(|| ValidationRule {
    name: "LOCALE_REGEX",
    regex: Regex::new(r"^[a-z]{2,3}(-[A-Z]{2})?$").unwrap()
});

pub static ROLE_NAME_REGEX: LazyLock<ValidationRule> = LazyLock::new(|| ValidationRule {
    name: "ROLE_NAME_REGEX",
    regex: Regex::new(r"^[a-z][a-z0-9_-]{2,49}$").unwrap()
});

pub static USERNAME_REGEX: LazyLock<ValidationRule> = LazyLock::new(|| ValidationRule {
    name: "USERNAME_REGEX",
    regex: Regex::new(r"^(?=.{6,30}$)[a-z][a-z0-9](?:[._-]?[a-z0-9])*$").unwrap()
});
