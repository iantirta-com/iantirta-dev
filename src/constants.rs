
use std::path::PathBuf;
use std::sync::OnceLock;
use std::{env};

use crate::cachepath;

pub static IDEV_HOME_DIR: OnceLock<PathBuf> = OnceLock::new();
pub static IDEV_CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();


/// Returns a reference to the global home directory
pub fn get_home_dir() -> &'static PathBuf {
    IDEV_HOME_DIR.get_or_init(|| {
        let base_home: String = if env::consts::OS == "windows" {
            env::var("USERPROFILE").unwrap_or_else(|_| String::from("C\\"))
        } else {
            env::var("HOME").unwrap_or_else(|_| String::from("/"))
        };
        let mut path = PathBuf::from(base_home);
        path.push(".idev");

        let final_path = cachepath::get_or_create_path(path);
        cachepath::make_cachedir_tag(&final_path);

        final_path
    })
}


/// Returns a reference to the global configuration path, building on top of the home directory.
pub fn get_config_path() -> &'static PathBuf {
    IDEV_CONFIG_PATH.get_or_init(|| {
        let home = get_home_dir();
        home.join("config.json")
    })
}