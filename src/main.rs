mod constants;
mod term;

use std::{
    env, fs,
    io::{self, Result},
};

use directories::ProjectDirs;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    name: String,
}

fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let environment = env::var("ENVIRONMENT").map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read ENVIRONMENT: {}", e),
        )
    })?;
    let organization = env::var("ORGANIZATION").map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read ORGANIZATION: {}", e),
        )
    })?;
    if let Some(proj_dirs) = ProjectDirs::from(
        &environment,
        &organization,
        constants::project::PROJECT_NAME,
    ) {
        let config_dir = proj_dirs.config_dir();
        let config_file = fs::read_to_string(config_dir.join("mycli.toml"));
        let _config: Config = match config_file {
            Ok(file) => {
                let parsed_config: Config = toml::from_str(&file).unwrap();
                term::start_terminal(&parsed_config.name).expect("TODO: panic message");
                parsed_config
            }
            Err(_) => {
                term::start_first_time_terminal().expect("TODO: panic message");
                Config {
                    name: String::new(),
                }
            }
        };
    }
    Ok(())
}
