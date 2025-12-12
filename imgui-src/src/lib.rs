/// Location of the SDL 3 source code
#[cfg(not(windows))]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/dcimgui");
#[cfg(windows)]
pub const SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\dcimgui");