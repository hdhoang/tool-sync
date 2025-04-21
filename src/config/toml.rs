use std::fs;
use std::path::PathBuf;

use crate::config::schema::Config;
use crate::infra::err;

pub fn with_parsed_file<F: FnOnce(Config)>(
    config_path: PathBuf,
    proxy: Option<String>,
    on_success: F,
) {
    match parse_config(&config_path, proxy) {
        Ok(config) => {
            on_success(config);
        }
        Err(e) => {
            err::abort_with(format!(
                "Error parsing configuration at path {}: {e}",
                config_path.display(),
            ));
        }
    }
}

fn parse_config(
    config_path: &PathBuf,
    _proxy: Option<String>,
) -> Result<Config, Box<dyn core::error::Error>> {
    let contents = fs::read_to_string(config_path)?;
    Ok(facet_toml::from_str(&contents)?)
}
