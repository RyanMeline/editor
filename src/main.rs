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
        let action: Action = keymap::handle_event(read()?);
        match action {
            Action::InsertChar(c) => editor.insert_char(c),
            Action::DeleteCharBack => {},
            Action::DeleteCharForward => {},
            Action::InsertNewLine => editor.new_line(),
            Action::MoveCursor(dir) => editor.move_cursor(dir),
            Action::MoveStartOfLine => {},
            Action::MoveEndOfLine => {},
            Action::Save => {},
            Action::Open(str) => {},
            Action::Quit => { break 'main_loop },
            Action::SaveQuit => { //save stuff
                    break 'main_loop;
                },
            Action::None => continue,
        }
        // if poll(Duration::from_millis(500))? {
            
        // }
    }
    Ok(())
}
