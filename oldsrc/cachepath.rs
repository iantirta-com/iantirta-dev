use std::path::{Path, PathBuf};
use std::{fs};


static CACHEDIR_CONTENT: &str = r#"Signature: 8a477f597d28d172789f06886806bc55
# This file is a cache directory tag created by iantirta-dev.
# For information about cache directory tags, see:
#	http://www.brynosaurus.com/cachedir/
"#;


pub fn get_or_create_path<P: AsRef<Path>>(pathname: P) -> PathBuf {
    let path: PathBuf = pathname.as_ref().to_path_buf();

    if !path.exists() {
        fs::create_dir_all(&path)
            .unwrap_or_else(|_| panic!("Failed to create directory: {}", path.display()));
    }

    path
}


// Make Cache dir tag on a dir given
pub fn make_cachedir_tag(dir: &Path) {
    let cachedir_path = dir.join("CACHEDIR.TAG");
    if !cachedir_path.exists() {
        fs::write(cachedir_path, CACHEDIR_CONTENT)
            .expect("Failed to write CACHEDIR.TAG file");
    }
}
