use semver::Version;
use std::process::{Command, Output};
use thiserror::Error;
use version_compare::Cmp;
use which::which_in;

#[cfg(feature = "pyo3")]
include!("python_bindings.rs");

#[derive(Error, Debug)]
pub enum LatestVersionError {
    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Failed to execute command {0}: {1}")]
    CommandExecutionError(String, std::io::Error),

    #[error("Version extraction failed: {0}")]
    VersionExtractionError(String),

    #[error("Failed to find executable paths")]
    PathFindingError(String),

    #[error("Failed to parse version: {0}")]
    VersionParsingError(#[from] semver::Error),
}

#[derive(Debug, Clone)]
pub struct ExecutableInfo {
    pub path: String,
    pub version: String,
}

pub fn find_executables(command: &str) -> Result<Vec<String>, LatestVersionError> {
    let path =
        std::env::var("PATH").map_err(|e| LatestVersionError::PathFindingError(e.to_string()))?;

    let mut executables = Vec::new();

    for dir in path.split(std::path::MAIN_SEPARATOR) {
        if dir.is_empty() {
            continue;
        }

        let dir_path = std::path::Path::new(dir);

        if let Ok(found) = which_in(command, Some(dir_path), dir_path) {
            if let Some(found_str) = found.to_str() {
                executables.push(found_str.to_string());
            }
        }
    }

    if executables.is_empty() {
        return Err(LatestVersionError::CommandNotFound(command.to_string()));
    }

    Ok(executables)
}

pub fn extract_version(output: &str) -> Option<String> {
    // Try to extract semantic version (x.y.z format)
    let semver_pattern =
        regex::Regex::new(r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)").unwrap();

    if let Some(captures) = semver_pattern.captures(output) {
        return Some(format!(
            "{}.{}.{}",
            &captures["major"], &captures["minor"], &captures["patch"]
        ));
    }

    // Try to extract major.minor format
    let minor_pattern = regex::Regex::new(r"(?P<major>\d+)\.(?P<minor>\d+)").unwrap();

    if let Some(captures) = minor_pattern.captures(output) {
        return Some(format!("{}.{}.0", &captures["major"], &captures["minor"]));
    }

    // Try to extract just major version
    let major_pattern = regex::Regex::new(r"(?P<major>\d+)").unwrap();

    if let Some(captures) = major_pattern.captures(output) {
        return Some(format!("{}.0.0", &captures["major"]));
    }

    None
}

pub fn get_version(executable_path: &str) -> Result<ExecutableInfo, LatestVersionError> {
    let mut command = Command::new(executable_path);
    command.arg("--version");

    let output: Output = command
        .output()
        .map_err(|e| LatestVersionError::CommandExecutionError(executable_path.to_string(), e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let combined_output = format!("{}{}", stdout, stderr);

    if let Some(version_str) = extract_version(&combined_output) {
        Ok(ExecutableInfo {
            path: executable_path.to_string(),
            version: version_str,
        })
    } else {
        // Try other version flags if --version failed
        for flag in ["-v", "-V", "version"] {
            let mut command = Command::new(executable_path);
            command.arg(flag);

            match command.output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let combined_output = format!("{}{}", stdout, stderr);

                    if let Some(version_str) = extract_version(&combined_output) {
                        return Ok(ExecutableInfo {
                            path: executable_path.to_string(),
                            version: version_str,
                        });
                    }
                }
                Err(_) => continue,
            }
        }

        Err(LatestVersionError::VersionExtractionError(
            "No version information found".to_string(),
        ))
    }
}

pub fn find_latest_version(
    info_list: Vec<ExecutableInfo>,
) -> Result<ExecutableInfo, LatestVersionError> {
    let mut latest_info = None;

    for info in info_list {
        match Version::parse(&info.version) {
            Ok(parsed_version) => match &latest_info {
                None => latest_info = Some(info),
                Some(latest) => match Version::parse(&latest.version) {
                    Ok(latest_version) => {
                        if parsed_version > latest_version {
                            latest_info = Some(info);
                        }
                    }
                    Err(_) => {
                        latest_info = Some(info);
                    }
                },
            },
            Err(_) => {
                // Fallback to flexible version comparison
                match &latest_info {
                    None => latest_info = Some(info),
                    Some(latest) => {
                        match version_compare::compare(&info.version, &latest.version) {
                            Ok(Cmp::Gt) => latest_info = Some(info),
                            _ => continue,
                        }
                    }
                }
            }
        }
    }

    latest_info.ok_or(LatestVersionError::VersionExtractionError(
        "No valid versions found".to_string(),
    ))
}

pub fn find_latest_command(command: &str) -> Result<ExecutableInfo, LatestVersionError> {
    let executables = find_executables(command)?;

    let mut info_list = Vec::new();

    for executable in executables {
        match get_version(&executable) {
            Ok(info) => info_list.push(info),
            Err(_) => continue,
        }
    }

    if info_list.is_empty() {
        return Err(LatestVersionError::VersionExtractionError(format!(
            "No version information found for command '{}'",
            command
        )));
    }

    find_latest_version(info_list)
}

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
