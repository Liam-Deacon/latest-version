#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_version_parsing() {
        let output = "Python 3.11.4";
        let version = extract_version(output);
        assert_eq!(version, Some("3.11.4".to_string()));
    }

    #[test]
    fn test_major_minor_version_parsing() {
        let output = "Node.js v18.16.0";
        let version = extract_version(output);
        assert_eq!(version, Some("18.16.0".to_string()));
    }

    #[test]
    fn test_major_version_parsing() {
        let output = "Git version 2";
        let version = extract_version(output);
        assert_eq!(version, Some("2.0.0".to_string()));
    }

    #[test]
    fn test_version_comparison() {
        let info1 = ExecutableInfo {
            path: "/usr/bin/python3".to_string(),
            version: "3.10.0".to_string(),
        };

        let info2 = ExecutableInfo {
            path: "/usr/local/bin/python3".to_string(),
            version: "3.11.0".to_string(),
        };

        let latest = find_latest_version(vec![info1, info2]).unwrap();
        assert_eq!(latest.path, "/usr/local/bin/python3");
        assert_eq!(latest.version, "3.11.0");
    }

    #[test]
    fn test_fallback_version_comparison() {
        let info1 = ExecutableInfo {
            path: "/usr/bin/java".to_string(),
            version: "1.8.0_302".to_string(),
        };

        let info2 = ExecutableInfo {
            path: "/usr/local/bin/java".to_string(),
            version: "11.0.16".to_string(),
        };

        let latest = find_latest_version(vec![info1, info2]).unwrap();
        assert_eq!(latest.path, "/usr/local/bin/java");
        assert_eq!(latest.version, "11.0.16");
    }
}
