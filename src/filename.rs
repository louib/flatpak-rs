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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_file_path_matches() {
        assert!(is_reverse_dns("com.example.appName.yaml"));
        assert!(is_reverse_dns("COM.EXAMPLE.APPNAME.YAML"));
        assert!(is_reverse_dns("io.github.user.repo.Devel.yaml"));
        assert!(is_reverse_dns("/path/to/com.example.appName.yaml"));
        assert!(is_reverse_dns("/path/to/com.example.appName.yml"));
        assert!(is_reverse_dns("/path/to/com.example.department.product.yaml"));
        assert!(is_reverse_dns(
            "/path/to/com.example.department.name-of-product.yaml"
        ));
        assert!(is_reverse_dns("contrib/linux/com.dosbox_x.DOSBox-X.yaml"));
        assert!(!is_reverse_dns(
            "/tmp/com.github.flathub.org.freedesktop.LinuxAudio.Plugins.WolfShaper/flathub.json"
        ));
        assert!(!is_reverse_dns("Firefox-62.0.3.update.json"));
        assert!(!is_reverse_dns("/path/to/file.yaml"));
        assert!(!is_reverse_dns("/path/to/file.json"));
        assert!(!is_reverse_dns("/path/to/___432423fdsf.json"));
        assert!(!is_reverse_dns("/path/to/example.com.json"));
        assert!(!is_reverse_dns("/path/to/example.com.json."));
        assert!(!is_reverse_dns(""));
        assert!(!is_reverse_dns("/////////////"));
    }
}
