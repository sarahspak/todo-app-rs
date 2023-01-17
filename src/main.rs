use ncurses::*;
use std::cmp::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id) {
        // disallow nested lists
        assert!(self.list_curr.is_none(), "nested lists are not allowed");
        self.list_curr = Some(id);
    }

    fn render_text(&mut self, text: &str, pair: i16) {
        // move the cursor to row,col
        mv(self.row as i32, self.col as i32);
        // set the color pair
        attron(COLOR_PAIR(pair));
        // render the text
        addstr(text);
        // disable color
        attroff(COLOR_PAIR(pair));
        // move the cursor to the next row
        self.row += 1;
    }

    fn list_element(&mut self, text: &str, id: Id) -> bool {
        let id_curr = self.list_curr.expect("list element outside of list");

        self.render_text(text, {
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });
        return false;
    }

    fn end_list(&mut self) {
        self.list_curr = None;
    }

    fn end(&mut self) {}
}

enum Focus {
    Todo,
    Done,
}

impl Focus {
    fn toggle(self) -> Self {
        match self {
            Focus::Todo => Focus::Done,
            Focus::Done => Focus::Todo,
        }
    }
}
fn list_up(list: &Vec<String>, list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1;
    } else if *list_curr == 0 {
        // start from the bottom
        *list_curr = list.len() - 1;
    }
}
fn list_down(list: &Vec<String>, list_curr: &mut usize) {
    *list_curr = if *list_curr + 1 <= list.len() - 1 {
        min(*list_curr + 1, list.len() - 1)
    } else {
        0
    }
}

fn main() {
    /* Start ncurses. */
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    // first color is highlighter, second is text color under highlighter
    init_pair(REGULAR_PAIR, COLOR_GREEN, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_GREEN);

    let mut quit = false;
    // state of application
    let mut todos: Vec<String> = vec![
        "go to walgreens for pickup".to_string(),
        "go to b&h for return".to_string(),
        "go climbing".to_string(),
    ];
    // todo_curr is the position of our highlighter
    let mut todo_curr: usize = 0;
    let mut dones: Vec<String> = vec![
        "watch tsoding cli video".to_string(),
        "start new jupyter brazil package for sujeewa".to_string(),
    ];
    // done_curr is the position of our highlighter
    let mut done_curr: usize = 0;
    let mut focus = Focus::Todo;

    let mut ui = Ui::default();
    // event loop
    while !quit {
        erase();
        ui.begin(0, 0);
        {
            match focus {
                Focus::Todo => {
                    ui.render_text("TODO:", REGULAR_PAIR);
                    ui.begin_list(todo_curr);
                    for (index, todo) in todos.iter().enumerate() {
                        // check if length of todos == 0
                        if todos.len() == 0 {
                            ui.render_text("\n", REGULAR_PAIR);
                        }
                        ui.list_element(&format!("- [ ] {}", todo), index);
                    }
                    ui.end_list();
                }
                Focus::Done => {
                    ui.render_text("DONE:", REGULAR_PAIR);
                    ui.begin_list(done_curr);
                    for (index, done) in dones.iter().enumerate() {
                        ui.list_element(&format!("- [x] {}", done), index);
                    }
                    ui.end_list();
                }
            }
            // applies to both
            ui.render_text("--------------------------------", REGULAR_PAIR);
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            // go up
            'k' => match focus {
                Focus::Todo => list_up(&todos, &mut todo_curr),
                Focus::Done => list_up(&dones, &mut done_curr),
            },
            // 'k' => {
            //     if todo_curr > 0 {
            //         todo_curr -= 1;
            //     } else if todo_curr == 0 {
            //         // start from the bottom
            //         todo_curr = todos.len() - 1;
            //     }
            // }
            // go down
            'j' => match focus {
                Focus::Todo => list_down(&todos, &mut todo_curr),
                Focus::Done => list_down(&dones, &mut done_curr),
            },
            // 'j' => {
            //     todo_curr = if todo_curr + 1 <= todos.len() - 1 {
            //         min(todo_curr + 1, todos.len() - 1)
            //     } else {
            //         0
            //     }
            // }
            // hit enter to move things between focus
            '\n' =>  match focus {Focus::Todo => if (todo_curr < todos.len()) {
                dones.push(todos.remove(todo_curr));
            } Focus::Done => if (done_curr < dones.len()) {
                todos.push(dones.remove(done_curr));
            }
            },
            // '\n' => {
            //     if (todo_curr < todos.len()) {
            //         dones.push(todos.remove(todo_curr));
            //     }
            // }
            '\t' => focus = focus.toggle(),
            _ => (),
        }
    }
    getch();
    endwin();
}
