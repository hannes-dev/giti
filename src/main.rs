use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::color;
use std::io::{Write, stdout, stdin};
use std::process::Command;
use std::str;

struct File {
    path: String,
    added: bool,
}

fn main() {

    // run_interface();
    let files = parse_status();
    print_status(files);
}

fn parse_status() -> Vec<File> {
    let result = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output()
            .expect("failed to run git status");

    let mut files = Vec::new();
    
    for item in result.stdout.split(|num| num == &10) {
        if item.len() >= 2 {
            let (status, path) = item.split_at(3);
            files.push(File {
                path: String::from(str::from_utf8(path).expect("Unable to read path")),
                added: status[0] != 32, // 32 is a space
            });
        }
    }

    files
}

fn print_status(files: Vec<File>) {
    let red = color::Fg(color::Red);
    let reset = color::Fg(color::Reset);
    let green = color::Fg(color::Green);

    for file in files {
        let mut check = " ";
        let mut color = red;

        if file.added {
            check = "x";
            color = green;
        }
        
        println!("{}[{}] {}{}\r", color, check, file.path, reset);
    }
}

fn run_interface() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(stdout, "Hey there.\r").unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Up        => println!("<up>\r"),
            Key::Down      => println!("<down>\r"),
            _              => println!("none\r"),
        }
        
        stdout.flush().unwrap();
    }
}
