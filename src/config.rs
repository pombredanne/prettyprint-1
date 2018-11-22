use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use shell_words;

use dirs::PROJECT_DIRS;
use util::transpose;

pub fn config_file() -> PathBuf {
    env::var("PRETTYPRINT_CONFIG_PATH")
        .ok()
        .map(PathBuf::from)
        .filter(|config_path| config_path.is_file())
        .unwrap_or(PROJECT_DIRS.config_dir().join("config"))
}

pub fn get_args_from_config_file() -> Result<Vec<OsString>, shell_words::ParseError> {
    Ok(transpose(
        fs::read_to_string(config_file())
            .ok()
            .map(|content| get_args_from_str(&content)),
    )?.unwrap_or(vec![]))
}

pub fn get_args_from_env_var() -> Option<Result<Vec<OsString>, shell_words::ParseError>> {
    env::var("PRETTYPRINT_OPTS")
        .ok()
        .map(|s| get_args_from_str(&s))
}

fn get_args_from_str<'a>(content: &'a str) -> Result<Vec<OsString>, shell_words::ParseError> {
    let args_per_line = content
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("#"))
        .map(|line| shell_words::split(line))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(args_per_line
        .iter()
        .flatten()
        .map(|line| line.into())
        .collect())
}
