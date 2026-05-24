mod terminal;
mod editor;
mod keymap;

use terminal::TerminalGuard;
use editor::{
    Editor,
    Action,
};

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
    cursor::{
        MoveTo,
    },
};

fn setup_term() -> Result<()> {
    let _ = enable_raw_mode()?;
    let _ = execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn main() -> Result<()>{
    setup_term()?;
    let _guard = TerminalGuard;
    let mut editor = Editor::new();

    execute!(stdout(), MoveTo(0, 0))?;

    'main_loop: loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    if event.is_release() {
                        continue;
                    }
                    match event.code {
                        KeyCode::Esc => break 'main_loop,
                        KeyCode::Char(c) if event.modifiers.contains(KeyModifiers::CONTROL) => {
                            match c {
                                'q' => break 'main_loop,
                                _ => {},
                            }
                        }
                        KeyCode::Char(c) => { execute!(stdout(), Print(c.to_string()))?; },
                        KeyCode::Enter => { execute!(stdout(), Print("\n"))? },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
