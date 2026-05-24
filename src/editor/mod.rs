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

    fn open_file(filename: Option<String>) {
        //set filename to whats passed in
        //Load in the data
        //set cursor to 0,0
        //redraw screen

    }

    fn insert_char(c: char) {

    }

    fn delete_char() {

    }

    fn move_cursor(dir: Direction) {
        match {
            Direction::Left => {
                if self.cursor_row == 0 {
                    execute!(stdout(), MoveTo(cursor_row, cursor_col));
                } else {
                    execute!(stdout(), MoveTo(cursor_row - 1, cursor_col));
                }
            },
            Direction::Right => {
                if self.cursor_row == line[cursor_col].len()-1 {
                    execute!(stdout(), MoveTo(cursor_row, cursor_col));
                } else {
                    execute!(stdout(), MoveTo(cursor_row - 1, cursor_col));
                }
            },
            Direction::Down => {
                if self.cursor_col == lines.len()-1 {
                    execute!(stdout(), MoveTo(cursor_row, cursor_col));
                } else {
                    execute!(stdout(), MoveTo(cursor_row, cursor_col - 1));
                }
            },
            Direction::Up => {
                if self.cursor_col == 0 {
                    execute!(stdout(), MoveTo(cursor_row, cursor_col));
                } else {
                    execute!(stdout(), MoveTo(cursor_row, cursor_col + 1));
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
