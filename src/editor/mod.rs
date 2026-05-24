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
        DisableBlinking,
        EnableBlinking,
        RestorePosition,
        SavePosition,
        MoveDown,
        MoveLeft,
        MoveRight,
        MoveUp,
    },
};

pub struct Editor {
    lines:          Vec<String>, //lines[cursor_row] is current row values
    cursor_line_no:     usize, //line number
    cursor_column:     usize, //what side to side
    scroll_offset:  usize,  //for whats showing on the screen
    unsaved:        bool,
    filename:       Option<String>, //option bc it can be empty
    // more stuff for later like undo redo stacks
    //editor modes (if im going for a vim like thing)
}

struct Line {
    buf: [char; 256],
    gap_start: i16,
    gap_end: i16,
    prev_line: *mut Line,
    next_line: *mut Line,
    //Pointers to prev and next lines (null if first line, null if last line)
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            cursor_line_no : 0,
            cursor_column : 0,
            scroll_offset : 0,
            unsaved : false,
            filename: None,
            lines: vec![String::new()],
        }
    }

    fn redraw_screen() { //draw text inside screen bounds
        //check 
    }

    fn open_file(&mut self, filename: Option<String>) {
        
        //need to have it check if the filename exists in the current directory (even if it gives a directory)
        //Can use option for that probably
        //Option in params is for if the string is entered in the first place
        //change
        
        //set filename to whats passed in
        //Load in the data
        //set cursor to 0,0
        //redraw screen

    }

    pub fn insert_char(&mut self, c: char) {
        execute!(stdout(), Print(c.to_string())).unwrap();
        self.cursor_column += 1;
    }

    pub fn new_line(&mut self) {
        execute!(stdout(), Print("\n")).unwrap();
        self.lines.push("".to_string());
        self.cursor_line_no += 1;
    }

    fn delete_char(&mut self) {

    }

    pub fn move_cursor(&mut self, dir: Direction) {

        //Need to do checks when going up and down to check line length
        //to stop jumping down to somewhere the line doesnt exist

        let c_line_no: u16 = self.cursor_line_no.try_into().unwrap();
        let mut c_column: u16 = self.cursor_column.try_into().unwrap(); 
        
        match dir {
            Direction::Left => {
                if self.cursor_column == 0 {
                    //do nothing
                    //execute!(stdout(), MoveLeft(1)).unwrap();
                } else {
                    self.cursor_column -= 1;
                    //execute!(stdout(), MoveLeft(1)).unwrap();
                    execute!(stdout(), MoveTo(c_column - 1, c_line_no)).unwrap();
                }
            },
            Direction::Right => { //Bugs in the code here
                execute!(stdout(), Print("test")).unwrap();
                // if self.cursor_column == self.lines[self.cursor_line_no].len()-1 {
                //     //do nothing
                //     execute!(stdout(), MoveRight(1)).unwrap();
                // } else {
                    self.cursor_column += 1;
                    //execute!(stdout(), MoveTo(c_line_no, c_column + 1)).unwrap();
                    execute!(stdout(), MoveRight(1)).unwrap();
                // }
            },
            Direction::Up => {
                if self.cursor_line_no == 0 { //at top
                    //do nothing
                    execute!(stdout(), MoveUp(1)).unwrap();
                } else {
                    //might need to do -1 after the len()
                    if self.lines[self.cursor_line_no-1].len() < self.cursor_column {
                        self.cursor_column = self.lines[self.cursor_line_no-1].len();
                    }
                    self.cursor_line_no -= 1;
                    //execute!(stdout(), MoveTo(c_line_no - 1, c_column)).unwrap();
                    execute!(stdout(), MoveUp(1)).unwrap();
                }
            },
            Direction::Down => {
                if self.cursor_column == self.lines.len()-1 { //at bottom
                    //do nothing
                    execute!(stdout(), MoveDown(1)).unwrap();
                } else {
                    //might need to do -1 after the len()
                    if self.lines[self.cursor_line_no+1].len() < self.cursor_column {
                        self.cursor_column = self.lines[self.cursor_line_no+1].len();
                    }
                    self.cursor_line_no += 1;
                    //execute!(stdout(), MoveTo(c_line_no + 1, c_column)).unwrap();
                    execute!(stdout(), MoveDown(1)).unwrap();
                }
            },
        };
    }

    fn save() {

    }
}

impl Line {
    fn move_buffer(&mut self, dist: i16) {
        //Move buffer left or right (neg or pos)
        //Can just call this when typing starts
        //like have a check when typing starts 
        // (am I in the buffer right now?) [if no: move buffer] [else: nothing]
        //each line will have its own buffer
        //need to move all text that was in buffer to the left or right depending on direction
    }

    fn get_line(&mut self) -> String {
        //get before and after the buffer, concat together, return

        let x = String::from("oaiwnd");
        x
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
    SaveQuit,
    None,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
