pub mod logger;

pub mod prelude {
    pub use camino::{Utf8Path, Utf8PathBuf};
    pub use cargo_metadata::Metadata;
    pub use compact_str::CompactString as XString;
    pub use duct::cmd;
    pub use eyre::{Context, Result};
    pub use indexmap::IndexMap;
    pub use jiff::Timestamp;
    pub use serde::{Deserialize, Serialize};

    pub use cargo_metadata;
    pub use duct;
    pub use indexmap;
    pub use serde;
    pub use serde_json;
}
