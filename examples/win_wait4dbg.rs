#[cfg(windows)]
use dbgtools::win::{debugger_output, wait_for_debugger};

#[cfg(windows)]
fn wininit() {
  println!("Waiting for a debugger to attach ..");
  wait_for_debugger();
  println!("Debugger attached!");

  debugger_output("Hello, debugger!\n");
}

fn main() {
  #[cfg(windows)]
  wininit();

  #[cfg(not(windows))]
  println!("Not really a useful non-Windows test..");
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
