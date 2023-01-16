use termion::{color, style, terminal_size};
use termion::raw::IntoRawMode;
use std::io::{stdout, Write, stdin};
use termion::event::Key;
use termion::input::TermRead;


fn main() {
    let mut stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
        _ => {}
        }
    }
    println!("{:?}", terminal_size());
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue and Bold{}", style::Bold, style::Reset);
    println!("{}Just plain Italic", style::Italic);
}
