use std::io;

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{backend::CrosstermBackend, Terminal};

mod setup;
mod setup_manager;
mod user_interface;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    let mut user_interface = user_interface::UserInterface::new();
    let res = user_interface::run_app(&mut terminal, user_interface);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
