use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REVERSE_DNS_FILENAME_REGEX: Regex = Regex::new(
        r"[a-z][a-z][a-z]*\.[a-z][0-9a-zA-Z_\-]+\.[a-z][0-9a-zA-Z_\-]+(\.[a-z][0-9a-zA-Z_\-]+)*\.(json|yaml|yml)$"
    ).unwrap();
}

pub fn is_reverse_dns(path: &str) -> bool {
    let path = path.to_lowercase();
    REVERSE_DNS_FILENAME_REGEX.is_match(&path)
}

pub fn extension_is_valid(path: &str) -> bool {
    let path = path.to_lowercase();
    is_yaml(&path) || is_json(&path)
}

pub fn is_yaml(path: &str) -> bool {
    path.ends_with(".yaml") || path.ends_with(".yml")
}

pub fn is_json(path: &str) -> bool {
    path.ends_with(".json")
}
