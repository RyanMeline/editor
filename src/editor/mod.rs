use std::{
    convert::{TryFrom, TryInto},
    io::{self, Result, Write, stdout},
    time::Duration,
};

use crossterm::{
    cursor::{
        DisableBlinking, EnableBlinking, Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp,
        RestorePosition, SavePosition, Show,
    },
    event::{Event, KeyCode, KeyModifiers, poll, read},
    execute,
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};

pub struct Editor {
    lines: Vec<Line>,      //lines[cursor_row] is current row values
    cursor_line_no: usize, //line number
    cursor_column: usize,  //what side to side
    scroll_offset: usize, //for whats showing on the screen, number of lines that are off the top of the screen
    unsaved: bool,
    filename: Option<String>, //option bc it can be empty
                              // more stuff for later like undo redo stacks
                              //editor modes (if im going for a vim like thing)
}

struct Line {
    buf: [char; 256],
    gap_start: u16,
    gap_end: u16,
    text_length: u16,
    //not used atm because I don't know how to do the box thing
    //prev_line: Option<*mut Line>,
    //next_line: Option<*mut Line>,
    //Pointers to prev and next lines (null if first line, null if last line)
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            cursor_line_no: 0,
            cursor_column: 0,
            scroll_offset: 0,
            unsaved: false,
            filename: None,
            lines: vec![Line::new()],
        }
    }

    /// Updates position tracker on the bottom left of the terminal
    pub fn update_position_tracker(&self) {
        let (cols, rows) = crossterm::terminal::size().unwrap();
        let target_col = cols.saturating_sub(1);
        let target_row = rows.saturating_sub(1);
        execute!(stdout(), Hide).unwrap();
        execute!(stdout(), MoveTo(target_col, target_row)).unwrap();
        execute!(stdout(), Print("^".to_string())).unwrap();
        execute!(
            stdout(),
            MoveTo(
                self.cursor_column.try_into().unwrap(),
                self.cursor_line_no.try_into().unwrap()
            )
        )
        .unwrap();
        execute!(stdout(), Show).unwrap();
    }

    /// Redraws current line
    fn redraw_line(&self) {
        //Hide cursor and move to the start of the line
        execute!(stdout(), Hide).unwrap();
        execute!(stdout(), MoveTo(0, self.cursor_line_no as u16)).unwrap();
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
        execute!(stdout(), Print(self.lines[self.cursor_line_no].get_line())).unwrap();
        execute!(
            stdout(),
            MoveTo(self.cursor_column as u16, self.cursor_line_no as u16)
        )
        .unwrap();
        execute!(stdout(), Show).unwrap();
    }

    /// Redraws entire screen
    fn redraw_screen(&self) {
        //draw text inside screen bounds
        execute!(stdout(), Hide).unwrap();

        //Come up with some logic to determine start pos
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        //this can probably be changed to redraw after cursor
        execute!(stdout(), Clear(ClearType::All)).unwrap();

        for line in &self.lines {
            execute!(stdout(), Print(line.get_line())).unwrap();
            execute!(stdout(), Print("\n")).unwrap();
        }
        execute!(
            stdout(),
            MoveTo(self.cursor_column as u16, self.cursor_line_no as u16)
        )
        .unwrap();
        execute!(stdout(), Show).unwrap();
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

    /// Inserts character at current cursor location
    ///
    /// # Arguments
    /// * `c` - Character to be entered
    ///
    /// # Examples
    /// ```
    /// editor.insert_char("a");
    /// ```
    pub fn insert_char(&mut self, c: char) {
        //execute!(stdout(), Print(c.to_string())).unwrap();
        self.lines[self.cursor_line_no].insert_char(c, self.cursor_column.try_into().unwrap());
        self.cursor_column += 1;
        self.redraw_line();
    }

    /// Enters a new line
    /// Moves text after cursor to the new line
    pub fn new_line(&mut self) {
        //check if gap_end is == buf.len()-1, if its *not*, take all the text after it, and push
        //that to a new line
        //If the buffer's end index is at the end of the line (enter is pressed at the end of the
        //line)
        if self.lines[self.cursor_line_no as usize].gap_end
            == (self.lines[self.cursor_line_no as usize].buf.len() - 1)
                .try_into()
                .unwrap()
        {
            let l_new = Line::new();
            self.lines.insert(self.cursor_line_no + 1, l_new);
            self.cursor_line_no += 1;
            self.cursor_column = 0;
            self.redraw_screen();
        } else {
            // take all the text from gap_end to line_end, copy it, delete it (move buffer end to end
            //of line), make a new line, add that text to the new line
            //get text between gap_end and line_end
            let temp_str: String = self.lines[self.cursor_line_no].get_text_after_buffer();
            //move buffer end to line_end
            self.lines[self.cursor_line_no].gap_end = (self.lines[self.cursor_line_no].buf.len()
                - 1)
            .try_into()
            .unwrap();
            self.lines[self.cursor_line_no].text_length -= temp_str.chars().count() as u16;
            //make a new line
            let l_new = Line::new();
            self.lines.insert(self.cursor_line_no + 1, l_new);
            self.cursor_line_no += 1;
            self.cursor_column = 0;
            //insert previously coppied text into new line
            for c in temp_str.chars() {
                self.lines[self.cursor_line_no]
                    .insert_char(c, self.cursor_column.try_into().unwrap());
                self.cursor_column += 1;
            }
            execute!(stdout(), Print(&temp_str)).unwrap();
            self.lines[self.cursor_line_no].move_buffer(0);
            self.cursor_column = 0;
            self.redraw_screen();
        }
    }

    /// Deletes character infront of cursor
    /// Moves gap buffer start point ontop of character before buffer, redraws line
    pub fn delete_char(&mut self) {
        if self.cursor_column == 0 {
            if self.cursor_line_no == 0 {
                return;
            }
            self.delete_line_backward();
            return;
        }
        self.lines[self.cursor_line_no].move_buffer(self.cursor_column.try_into().unwrap());
        self.cursor_column -= 1;
        execute!(
            stdout(),
            MoveTo(
                self.cursor_column as u16,
                self.cursor_line_no.try_into().unwrap()
            )
        )
        .unwrap();
        self.lines[self.cursor_line_no].gap_start -= 1;
        self.lines[self.cursor_line_no].text_length -= 1;
        self.redraw_line()
    }

    /// Deletes character after cursor position
    /// Moves gap buffer end point over character, redraws line
    pub fn delete_char_forward(&mut self) {
        if self.lines[self.cursor_line_no].gap_end as usize
            == self.lines[self.cursor_line_no].buf.len() - 1
        {
            if self.cursor_line_no as usize == self.lines.len() - 1 {
                return;
            }
            self.delete_line_forward();
            return;
        }
        self.lines[self.cursor_line_no].move_buffer(self.cursor_column.try_into().unwrap());
        self.lines[self.cursor_line_no].gap_end += 1;
        self.lines[self.cursor_line_no].text_length -= 1;
        self.redraw_line()
    }

    /// Moves cursor in entered direction
    ///
    /// # Arguments
    /// * `dir`: `Direction` - Direction of movement
    ///
    /// ```
    /// enum Direction
    ///     Direction::Left
    ///     Direction::Right
    ///     Direction::Up
    ///     Direction::Down
    /// ```
    pub fn move_cursor(&mut self, dir: Direction) {
        let c_line_no: u16 = self.cursor_line_no.try_into().unwrap();
        let c_column: u16 = self.cursor_column.try_into().unwrap();

        match dir {
            Direction::Left => {
                if self.cursor_column != 0 {
                    self.cursor_column -= 1;
                    execute!(stdout(), MoveTo(c_column - 1, c_line_no)).unwrap();
                    self.lines[self.cursor_line_no]
                        .move_buffer(self.cursor_column.try_into().unwrap());
                    self.redraw_line();
                }
            }
            Direction::Right => {
                if self.cursor_column != self.lines[self.cursor_line_no].text_length as usize {
                    self.cursor_column += 1;
                    execute!(stdout(), MoveTo(c_column + 1, c_line_no)).unwrap();
                    self.lines[self.cursor_line_no]
                        .move_buffer(self.cursor_column.try_into().unwrap());
                    self.redraw_line();
                }
            }
            Direction::Up => {
                if self.cursor_line_no != 0 {
                    if self.lines[self.cursor_line_no - 1].text_length < c_column {
                        self.cursor_column = self.lines[self.cursor_line_no - 1]
                            .text_length
                            .try_into()
                            .unwrap();
                    }
                    self.cursor_line_no -= 1; //swap line no and column
                    execute!(
                        stdout(),
                        MoveTo(
                            self.cursor_column.try_into().unwrap(),
                            self.cursor_line_no.try_into().unwrap()
                        )
                    )
                    .unwrap();
                }
            }
            Direction::Down => {
                if self.cursor_line_no != self.lines.len() - 1 {
                    if self.lines[self.cursor_line_no + 1].text_length < c_column {
                        self.cursor_column = self.lines[self.cursor_line_no + 1]
                            .text_length
                            .try_into()
                            .unwrap();
                    }
                    self.cursor_line_no += 1; //swap line no and column
                    execute!(
                        stdout(),
                        MoveTo(
                            self.cursor_column.try_into().unwrap(),
                            self.cursor_line_no.try_into().unwrap()
                        )
                    )
                    .unwrap();
                }
            }
        };
    }

    fn save(&mut self) {}

    /// Deletes current line
    /// Moves text to line above
    fn delete_line_backward(&mut self) {
        let temp = self.lines[self.cursor_line_no].get_line();
        self.lines.remove(self.cursor_line_no);
        self.cursor_line_no -= 1;
        let temp_column = self.lines[self.cursor_line_no].text_length;
        self.cursor_column = temp_column as usize;
        for c in temp.chars() {
            self.insert_char(c);
        }
        self.cursor_column = temp_column as usize;
        self.redraw_screen();
    }

    /// Deletes line below current
    /// Moves next line text to current line
    fn delete_line_forward(&mut self) {
        let temp = self.lines[self.cursor_line_no + 1].get_line();
        let temp_column = self.cursor_column;
        self.lines.remove(self.cursor_line_no + 1);
        for c in temp.chars() {
            self.insert_char(c);
        }
        self.cursor_column = temp_column as usize;
        self.redraw_screen();
    }
}

impl Line {
    fn new() -> Self {
        Line {
            buf: ['\0'; 256],
            gap_start: 0,
            gap_end: 255, //gap starts out as the entire line and fills that way. This is since I
            //have a set size buffer rather than dynamically sized
            text_length: 0,
        }
    }

    /// Inserts char into line at given position
    ///
    /// # Arguments
    /// * `c` - Character to be entered
    /// * `current_index` - Cursor position within the line
    fn insert_char(&mut self, c: char, current_index: u16) {
        if self.gap_start == self.gap_end {
            //do something about if gap is full
        }

        self.move_buffer(current_index);
        self.buf[self.gap_start as usize] = c;
        self.gap_start += 1;
        self.text_length += 1;
    }

    /// Moves buffer start to current position, shifting displaced text to the other side of the buffer
    ///
    /// # Arguments
    /// * `new_start` - Index in the line the gap buffer start is moving to
    ///
    /// # Examples
    /// ```
    /// ['C','u','r','r','e','n','t',_,_,_,_,'g','a','p']
    /// Line::move_buffer(3);
    /// ['C','u','r',_,_,_,_,'r','e','n','t','g','a','p']
    /// ```
    fn move_buffer(&mut self, new_start: u16) {
        //where the char is input is to the left of where the gap is
        let init_gap_start = self.gap_start;
        if new_start < self.gap_start {
            //grab everything between the two
            //decrease buffer end by the difference between the two
            //put everything after the buffer end ( or swap these two steps, so put at the end of
            //the buffer and move the buffer end. could iterate through one character at a time)
            //
            //iterate through one char at a time, shift buffer head left, copy char from buffer
            for i in (new_start..init_gap_start).rev() {
                self.gap_start -= 1;
                let c = self.buf[i as usize];
                self.buf[self.gap_end as usize] = c;
                self.gap_end -= 1;
            }
            //head, paste at buffer end, move buffer end left
        } else if new_start > self.gap_start {
            //new place is to the right of the gap head so it needs to move to new current_index
            //
            //iterate buffer to the right
            //buffer end += 1, copy char in buffer end, put into buffer start, buffer start += 1
            for i in init_gap_start..new_start {
                self.gap_end += 1;
                let c = self.buf[self.gap_end as usize];
                self.buf[self.gap_start as usize] = c;
                self.gap_start += 1;
            }
        }
        //if new_start < gap_start, move everything between the two to the right
        //if new_start > gap_start, move everything between gap_end and buf.len() to the left
        //Move buffer to new start position
        //Can just call this when typing starts
        //like have a check when typing starts
        // (am I in the buffer right now?) [if no: move buffer] [else: nothing]
        //each line will have its own buffer
        //need to move all text that was in buffer to the left or right depending on direction
    }

    /// Returns a String containing text from both sides of the gap buffer
    ///
    /// # Returns
    /// * `String`
    fn get_line(&self) -> String {
        //                               no + 1 since its not ..=
        let mut s1: String = self.buf[..self.gap_start as usize]
            .iter()
            .filter(|&&c| c != '\0')
            .collect();
        let s2: String = self.buf[(self.gap_end + 1) as usize..]
            .iter()
            .filter(|&&c| c != '\0')
            .collect();

        s1.push_str(&s2);
        s1
    }

    fn get_text_after_buffer(&self) -> String {
        let mut temp_str: String = String::new();
        for i in (self.gap_end as usize) + 1..self.buf.len() {
            temp_str.push_str(&self.buf[i].to_string());
        }
        temp_str
    }
}

//Actions that will be called by the keymap (what keys are used in the editor)
//Actions sent to main
//Main maps actions to editor functions
pub enum Action {
    InsertChar(char),
    DeleteCharBack,    //backspace
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
