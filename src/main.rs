mod terminal;
mod editor;
mod keymap;

use terminal::TerminalGuard;
use std::{
    time::Duration,
    io::{
        stdout, 
        Result, 
        self, 
        Write,
    },
};
use crossterm:: {
    execute,
    event:: {
        poll,
        Event,
        KeyCode,
        read,
    },
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    }
};

fn setup_term() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}
fn main() -> Result<()>{
    setup_term()?;
    let _guard = TerminalGuard;
    'main_loop: loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('q') => break 'main_loop,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
