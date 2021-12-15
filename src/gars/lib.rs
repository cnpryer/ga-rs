pub mod core;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub fn get_version() -> &'static str {
    VERSION.unwrap_or("unknown")
}
