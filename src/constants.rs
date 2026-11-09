use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::{env, fs};

// ---------------------------
// Public API constants
// ---------------------------


pub static IDEV_HOME: LazyLock<PathBuf> = LazyLock::new(|| idev_home());
pub static IDEV_CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| idev_config_path());
pub static IDEV_PROJECT_DIR: LazyLock<PathBuf> = LazyLock::new(|| idev_project_dir());


// ---------------------------
// Helper method const getter
// ---------------------------


const CACHEDIR_CONTENT: &str = r#"Signature: 8a477f597d28d172789f06886806bc55
# This file is a cache directory tag created by iantirta-dev.
# For information about cache directory tags, see:
#	http://www.brynosaurus.com/cachedir/
"#;


fn ensure_dir(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir_all(&path)
            .unwrap_or_else(|_| panic!("Failed to create directory: {}", path.display()));
    }
}


fn make_cachedir_tag(dirpath: &PathBuf) {
    let cachedir_path = dirpath.join("CACHEDIR.TAG");
    if !cachedir_path.exists() {
        fs::write(cachedir_path, CACHEDIR_CONTENT)
            .expect("Failed to write CACHEDIR.TAG file");
    }
}


fn home_dir() -> PathBuf {
    env::home_dir().expect("Failed to get home directory")
}


fn idev_home() -> PathBuf {
    let mut home = home_dir();
    home.push(".idev");
    ensure_dir(&home);
    make_cachedir_tag(&home);
    home
}


fn idev_config_path() -> PathBuf {
    idev_home().join(".idevconfig")
}

fn idev_project_dir() -> PathBuf {
    home_dir().join("IANTIRTACOM")
}


fn main() {
    println!("Idev Home dir: {:#?}", *IDEV_HOME);
    println!("Idev Config Path: {:#?}", *IDEV_CONFIG_PATH);
    println!("Home dir: {:#?}", home_dir());
}
