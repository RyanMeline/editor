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
    fn new(&self) -> Self {
        Self {

        }
    }

    fn redraw_screen() {

    }

    fn open_file(Option<String>) {
        //set filename to whats passed in
        //Load in the data
        //set cursor to 0,0
        //redraw screen

    }

    fn insert_char(c: char) {

    }

    fn delete_char() {

    }

    fn move_cursor(Direction) {

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