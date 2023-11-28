use std::io::{self, stdout};

use crossterm::{
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};

pub struct KeepTerminalState;

impl KeepTerminalState {
  pub fn new() -> io::Result<Self> {
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(Self)
  }
}

impl Drop for KeepTerminalState {
  fn drop(&mut self) {
    terminal::disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
  }
}
