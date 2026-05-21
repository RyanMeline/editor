use std::io::stdout;
use crossterm::{
    execute,
    terminal::{
        LeaveAlternateScreen,
        disable_raw_mode,
    }
};


pub struct TerminalGuard;

//destructor (called when main ends or panic happens)
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}


// Add the render here, call it at the beginning of each loop