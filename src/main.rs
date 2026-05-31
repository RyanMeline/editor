mod editor;
mod keymap;
mod terminal;

use editor::{Action, Editor};
use terminal::TerminalGuard;

use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyModifiers, poll, read},
    execute,
    style::Print,
    terminal::{
        DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::{
    io::{self, Result, Write, stdout},
    time::Duration,
};

fn setup_term() -> Result<()> {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    execute!(stdout(), DisableLineWrap).unwrap();
    Ok(())
}

fn main() -> Result<()> {
    setup_term().unwrap();
    let _guard = TerminalGuard;
    let mut editor = Editor::new();

    execute!(stdout(), MoveTo(0, 0))?;

    'main_loop: loop {
        let action: Action = keymap::handle_event(read()?);
        match action {
            Action::InsertChar(c) => editor.insert_char(c),
            Action::DeleteCharBack => editor.delete_char(),
            Action::DeleteCharForward => editor.delete_char_forward(),
            Action::InsertNewLine => editor.new_line(),
            Action::MoveCursor(dir) => editor.move_cursor(dir),
            Action::MoveStartOfLine => {}
            Action::MoveEndOfLine => {}
            Action::Save => {}
            Action::Open(str) => {}
            Action::Quit => break 'main_loop,
            Action::SaveQuit => {
                //save stuff
                break 'main_loop;
            }
            Action::None => continue,
        }
        editor.update_position_tracker();
        // if poll(Duration::from_millis(500))? {

        // }
    }
    Ok(())
}
