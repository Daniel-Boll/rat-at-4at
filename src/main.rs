use crossterm::event::{self, Event, KeyCode};
use rat_at_4at::utils::terminal_state::KeepTerminalState;
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
  let _lock = KeepTerminalState::new()?;

  let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

  let mut should_quit = false;
  while !should_quit {
    terminal.draw(ui)?;
    should_quit = handle_events()?;
  }

  Ok(())
}

fn handle_events() -> io::Result<bool> {
  if event::poll(std::time::Duration::from_millis(50))? {
    if let Event::Key(key) = event::read()? {
      if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
        return Ok(true);
      }
    }
  }
  Ok(false)
}

fn ui(frame: &mut Frame) {
  frame.render_widget(
    Paragraph::new("WIP").block(Block::default().title("4at").borders(Borders::ALL)),
    frame.size(),
  );
}
