use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::CrosstermBackend,
    Terminal,
};

pub type Term = Terminal<CrosstermBackend<io::Stdout>>;

pub fn init() -> Result<Term, io::Error> {
    let mut buf = stdout();
    execute!(buf, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(buf))?;
    term.hide_cursor()?;

    Ok(term)
}

pub fn restore(term: &mut Term) -> Result<(), io::Error> {
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    term.show_cursor()?;

    Ok(())
}
