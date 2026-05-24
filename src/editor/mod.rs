use std::{
    time::Duration,
    io::{
        stdout, 
        Result, 
        self, 
        Write,
    },
    convert::{
        TryInto,
        TryFrom,
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

pub struct Editor {
    lines:          Vec<String>,
    cursor_row:     usize,
    cursor_col:     usize,
    scroll_offset:  usize,  //for whats showing on the screen
    unsaved:        bool,
    filename:       Option<String>, //option bc it can be empty
    // more stuff for later like undo redo stacks
    //editor modes (if im going for a vim like thing)
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            cursor_row : 0,
            cursor_col : 0,
            scroll_offset : 0,
            unsaved : false,
            filename: None,
            lines: vec![String::new()],
        }
    }

    fn redraw_screen() {

    }

    fn open_file(&mut self, filename: Option<String>) {
        //set filename to whats passed in
        //Load in the data
        //set cursor to 0,0
        //redraw screen

    }

    fn insert_char(&mut self, c: char) {

    }

    fn delete_char(&mut self) {

    }

    fn move_cursor(&mut self, dir: Direction) {
        let c_row: u16 = self.cursor_row.try_into().unwrap();
        let c_col: u16 = self.cursor_col.try_into().unwrap(); 
        
        match dir {
            Direction::Left => {
                if self.cursor_row == 0 {
                    execute!(stdout(), MoveTo(c_row, c_col)).unwrap();
                } else {
                    execute!(stdout(), MoveTo(c_row - 1, c_col)).unwrap();
                }
            },
            Direction::Right => {
                if self.cursor_row == self.lines[self.cursor_col].len()-1 {
                    execute!(stdout(), MoveTo(c_row, c_col)).unwrap();
                } else {
                    execute!(stdout(), MoveTo(c_row - 1, c_col)).unwrap();
                }
            },
            Direction::Down => {
                if self.cursor_col == self.lines.len()-1 {
                    execute!(stdout(), MoveTo(c_row, c_col)).unwrap();
                } else {
                    execute!(stdout(), MoveTo(c_row, c_col - 1)).unwrap();
                }
            },
            Direction::Up => {
                if self.cursor_col == 0 {
                    execute!(stdout(), MoveTo(c_row, c_col)).unwrap();
                } else {
                    execute!(stdout(), MoveTo(c_row, c_col + 1)).unwrap();
                }
            },
        };
    }

    fn save() {

    }
}

//Actions that will be called by the keymap (what keys are used in the editor)
//Actions sent to main
//Main maps actions to editor functions
pub enum Action {
    InsertChar(char),
    DeleteCharBack,//backspace
    DeleteCharForward, //del key
    InsertNewLine,

    MoveCursor(Direction),
    MoveStartOfLine,
    MoveEndOfLine,

    Save,
    Open(String),

    Quit,

    None,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
