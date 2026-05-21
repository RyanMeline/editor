use std::io::stdout;
use crossterm::{
    execute,
    terminal::{
        LeaveAlternateScreen,
        disable_raw_mode,
    }
};


pub struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen);
        disable_raw_mode();
    }
}
