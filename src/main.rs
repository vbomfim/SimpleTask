mod app;

use app::App;
use color_eyre::Result;

use crossterm::event::DisableMouseCapture;
use crossterm::terminal::LeaveAlternateScreen;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use std::io::{self, Stderr};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = try_init()?;
    let result = App::new().run(&mut terminal);
    restore(terminal);
    result
}
pub type TerminalStdErr = Terminal<CrosstermBackend<Stderr>>;

pub fn try_init() -> io::Result<TerminalStdErr> {
    set_panic_hook();
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    Terminal::new(backend)
}
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        hook(info);
    }));
}
pub fn restore(terminal: TerminalStdErr) {
    if let Err(err) = try_restore(terminal) {
        // There's not much we can do if restoring the terminal fails, so we just print the error
        eprintln!("Failed to restore terminal: {err}");
    }
}
pub fn try_restore(mut terminal: TerminalStdErr) -> io::Result<()> {
    // disabling raw mode first is important as it has more side effects than leaving the alternate
    // screen buffer
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
