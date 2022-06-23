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

/// Transforms a URL into its reverse DNS equivalent.
///```
///let mut reverse_dns = flatpak_rs::reverse_dns::from_url("https://github.com/louib/flatpak-rs.git");
///assert_eq!(reverse_dns, "com.github.louib.flatpak-rs");
///reverse_dns =
///flatpak_rs::reverse_dns::from_url("https://gitlab.com/louib/flatpak-rs.git");
///assert_eq!(reverse_dns, "com.gitlab.louib.flatpak-rs");
///reverse_dns = flatpak_rs::reverse_dns::from_url("https://git.savannah.gnu.org/cgit/make.git");
///assert_eq!(reverse_dns, "org.gnu.savannah.git.cgit.make");
///reverse_dns = flatpak_rs::reverse_dns::from_url("https://gitlab.freedesktop.org/xorg/lib/libxmu");
///assert_eq!(reverse_dns, "org.freedesktop.gitlab.xorg.lib.libxmu");
///```
pub fn from_url(url: &str) -> String {
    if !url.starts_with("https://") {
        panic!("Only supports https urls: {}", url);
    }
    let mut sanitized_url = url[8..].to_string();

    if url.ends_with(".git") {
        // Removing the .git at the end of the url.
        // There has to be a better way to do this...
        // But rust has no negative index for the list
        // comprehension.
        sanitized_url.pop();
        sanitized_url.pop();
        sanitized_url.pop();
        sanitized_url.pop();
    }

    let mut repo_url_parts = sanitized_url.split("/");
    let domain = repo_url_parts.next().unwrap();
    let mut reversed_domain: String = "".to_string();

    let domain_parts = domain.split(".");
    for domain_part in domain_parts {
        if reversed_domain.len() == 0 {
            reversed_domain = domain_part.to_string();
        } else {
            reversed_domain = format!("{}.{}", domain_part, reversed_domain);
        }
    }

    let mut next_url_part = repo_url_parts.next();
    while next_url_part.is_some() {
        reversed_domain += ".";
        reversed_domain += next_url_part.unwrap();
        next_url_part = repo_url_parts.next();
    }
    reversed_domain
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
