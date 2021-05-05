pub use dbgtools_hexdump as hexdump;

#[cfg(windows)]
pub use dbgtools_win as win;

pub fn version() -> &'static str {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  VERSION
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
