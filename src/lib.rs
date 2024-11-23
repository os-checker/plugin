pub mod logger;

pub mod prelude {
    pub use camino::{Utf8Path, Utf8PathBuf};
    pub use cargo_metadata::Metadata;
    pub use compact_str::CompactString as XString;
    pub use eyre::{Context, Result};
    pub use indexmap::IndexMap;
}
