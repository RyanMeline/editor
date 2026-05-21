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
        KeyModifiers,
        read,
    },
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    style::Print,
};

fn setup_term() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn write_char(c: char) -> Result<()> {
    execute!(stdout(), Print(c.to_string()))?;
    stdout().flush()?;
    Ok(())
}

fn main() -> Result<()>{
    setup_term()?;
    let _guard = TerminalGuard;
    'main_loop: loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    if event.is_release() {
                        continue;
                    }
                    match event.code {
                        KeyCode::Esc => break 'main_loop,
                        // KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => break 'main_loop,
                        KeyCode::Char(c) if event.modifiers.contains(KeyModifiers::CONTROL) => {
                            match c {
                                'q' => break 'main_loop,
                                _ => {},
                            }
                        }
                        KeyCode::Char(c) => _ = write_char(c),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
