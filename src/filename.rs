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
