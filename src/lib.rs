pub mod logger;

pub mod prelude {
    pub use camino::{Utf8Path, Utf8PathBuf};
    pub use cargo_metadata::Metadata;
    pub use compact_str::CompactString as XString;
    pub use duct::cmd;
    pub use eyre::{Context, ContextCompat, Result};
    pub use indexmap::IndexMap;
    pub use jiff::Timestamp;
    pub use serde::{Deserialize, Serialize};

    pub use cargo_metadata;
    pub use duct;
    pub use indexmap;
    pub use serde;
    pub use serde_json;
}

#[macro_use]
extern crate tracing;

use prelude::*;
use std::fs;

/// Write Serializable data to a file. This function creates all parent
/// folders if they do not exist.
pub fn write_json<T: Serialize>(path: &Utf8Path, val: &T) -> Result<()> {
    let _span = error_span!("write_json", ?path).entered();

    let parent = path.parent().with_context(|| "No parent path found.")?;
    fs::create_dir_all(parent)?;

    serde_json::to_writer_pretty(fs::File::create(path)?, val)?;
    Ok(())
}

/// Get the list of repos, searching in the following order:
/// * the first CLI argument: a json path to a vec of repo string
/// * or read from the result of `os-checker config --list-repos`
pub fn repos() -> Result<Vec<String>> {
    let arg = std::env::args().nth(1);

    let text = match arg.as_deref() {
        Some(list_json) => {
            let path = Utf8Path::new(list_json);
            let _span = error_span!("repos", ?path).entered();
            fs::read_to_string(path)?
        }
        None => duct::cmd!("os-checker", "config", "--list-repos")
            .env_remove("RUST_LOG")
            .read()
            .with_context(|| "Failed to run `os-checker config --list-repos`")?,
    };

    info!(text);
    Ok(serde_json::from_str(&text)?)
}
