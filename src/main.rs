use clap::Parser;
use semver::Version;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Output};
use thiserror::Error;
use version_compare::Cmp;
use which::which;

#[derive(Error, Debug)]
enum LatestVersionError {
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
struct ExecutableInfo {
    path: String,
    version: String,
}

fn find_executables(command: &str) -> Result<Vec<String>, LatestVersionError> {
    let path =
        std::env::var("PATH").map_err(|e| LatestVersionError::PathFindingError(e.to_string()))?;

    let mut executables = Vec::new();

    for dir in path.split(std::path::MAIN_SEPARATOR) {
        if dir.is_empty() {
            continue;
        }

        let command_path = std::path::Path::new(dir).join(command);

        if command_path.is_file() && command_path.exists() {
            match command_path.metadata() {
                Ok(metadata) => {
                    let is_executable = metadata.permissions().mode() & 0o111 != 0;

                    if is_executable {
                        if let Some(found_str) = command_path.to_str() {
                            executables.push(found_str.to_string());
                        }
                    }
                }
                Err(_) => continue,
            }
        }
    }

    if executables.is_empty() {
        if let Ok(found) = which(command) {
            if let Some(found_str) = found.to_str() {
                executables.push(found_str.to_string());
            }
        } else {
            return Err(LatestVersionError::CommandNotFound(command.to_string()));
        }
    }

    Ok(executables)
}

fn extract_version(output: &str) -> Option<String> {
    let semver_pattern =
        regex::Regex::new(r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)").unwrap();

    if let Some(captures) = semver_pattern.captures(output) {
        return Some(format!(
            "{}.{}.{}",
            &captures["major"], &captures["minor"], &captures["patch"]
        ));
    }

    let minor_pattern = regex::Regex::new(r"(?P<major>\d+)\.(?P<minor>\d+)").unwrap();

    if let Some(captures) = minor_pattern.captures(output) {
        return Some(format!("{}.{}.0", &captures["major"], &captures["minor"]));
    }

    let major_pattern = regex::Regex::new(r"(?P<major>\d+)").unwrap();

    if let Some(captures) = major_pattern.captures(output) {
        return Some(format!("{}.0.0", &captures["major"]));
    }

    None
}

fn get_version(executable_path: &str) -> Result<ExecutableInfo, LatestVersionError> {
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

fn find_latest_version(
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
            Err(_) => match &latest_info {
                None => latest_info = Some(info),
                Some(latest) => match version_compare::compare(&info.version, &latest.version) {
                    Ok(Cmp::Gt) => latest_info = Some(info),
                    _ => continue,
                },
            },
        }
    }

    latest_info.ok_or(LatestVersionError::VersionExtractionError(
        "No valid versions found".to_string(),
    ))
}

fn find_latest_command(command: &str) -> Result<ExecutableInfo, LatestVersionError> {
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

#[derive(Parser, Debug)]
#[command(
    name = "latest-version",
    version = "0.1.0",
    about = "Find the latest version of commands across all available paths",
    long_about = None
)]
struct Args {
    /// Command to check for latest version
    #[arg(value_name = "COMMAND")]
    command: String,
}

fn main() -> std::process::ExitCode {
    let args = Args::parse();

    match find_latest_command(&args.command) {
        Ok(info) => {
            println!("{}", info.path);
            std::process::ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::ExitCode::FAILURE
        }
    }
}
