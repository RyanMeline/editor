//do something like
// let action = keymap::handle_event(read()?);
//in main
//where action is an Action enum and is then run through a match statement in main to call editor functions

use crate::editor::{ Action, Direction };

use std::io::stdout;
use crossterm:: {
    cursor,
    execute,
    style::Print,
    event::{
        Event,
        poll,
        KeyCode,
        KeyModifiers,
        read,
    }
};

pub fn handle_event(event: Event) -> Action {
    match event {
        Event::Key(event) => {
            if event.is_release() { return Action::None }
            match event.code {
                KeyCode::Esc => 
                    return Action::Quit,
                KeyCode::Char(c) 
                    if event.modifiers.contains(KeyModifiers::CONTROL) => {
                        match c {
                            'q' => return Action::Quit,
                            _ => {},
                        }
                    }
                KeyCode::Char(c) => { 
                    //execute!(stdout(), Print(c.to_string())).unwrap(); 
                    return Action::InsertChar(c)} //add a key
                KeyCode::Enter => {
                    //execute!(stdout(), Print("\n")).unwrap();
                    return Action::InsertNewLine; 
                }
                KeyCode::Backspace => 
                    return Action::DeleteCharBack,
                KeyCode::Delete => 
                    return Action::DeleteCharForward,
                KeyCode::Down => 
                    return Action::MoveCursor(Direction::Down),
                KeyCode::Up => 
                    return Action::MoveCursor(Direction::Up),
                KeyCode::Left => 
                    return Action::MoveCursor(Direction::Left),
                KeyCode::Right => 
                    return Action::MoveCursor(Direction::Right),
                _ => {}
            }
        },
        Event::Paste(str) => {},
        Event::Mouse(me) => {},
        Event::Resize(cols, rows) => {},
        _ => {}, //FocuseGained, FocusLost not implemented
    }

    Action::None
}

// match read()? {
//                 Event::Key(event) => {
//                     if event.is_release() {
//                         continue;
//                     }
//                     match event.code {
//                         KeyCode::Esc => break 'main_loop,
//                         KeyCode::Char(c) if event.modifiers.contains(KeyModifiers::CONTROL) => {
//                             match c {
//                                 'q' => break 'main_loop,
//                                 _ => {},
//                             }
//                         }
//                         KeyCode::Char(c) => { execute!(stdout(), Print(c.to_string()))?; },
//                         KeyCode::Enter => { execute!(stdout(), Print("\n"))? },
//                         _ => {}
//                     }
//                 }
//                 _ => {}
//             }