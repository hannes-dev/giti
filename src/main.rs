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
    to_add: bool,
}

fn main() {

    // run_interface();
    let files = parse_status();
    run_interface(files);
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
                added: status[0] != 32 && status[0] != 63, // 32 is a space, 63 is question mark
                to_add: status[0] != 32 && status[0] != 63,
            });
        }
    }

    files
}

fn print_status(files: &Vec<File>, selected: usize) {
    let red = color::Fg(color::Red);
    let reset = color::Fg(color::Reset);
    let green = color::Fg(color::Green);

    for (index, file) in files.iter().enumerate() {
        let (check, color) = if file.to_add {
            ("x", format!("{}", green))
        } else {
            (" ", format!("{}", red))
        };

        let select_marks = if index == selected {
            [">", "<"]
        } else {
            [" ", " "]
        };
        
        println!("{}{}[{}] {}{}{}\r", select_marks[0], color, check, file.path, reset, select_marks[1]);
    }
}

fn run_interface(mut files: Vec<File>) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut selected = 0;

    writeln!(stdout, "Hey there.\r").unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Esc       => break,
            Key::Up        => if selected > 0 {selected -= 1},
            Key::Down      => if selected < files.len() - 1 {selected += 1},
            Key::Char(' ') => files[selected].to_add = !files[selected].to_add,
            _              => (),
        }

        print_status(&files, selected);
        
        stdout.flush().unwrap();
    }

    commit_changes(files);
}

fn commit_changes(files: Vec<File>) {
    let mut add = Command::new("git");
    add.arg("add");

    let mut remove = Command::new("git");
    remove.arg("restore")
          .arg("--staged");

    let mut run_add = false;
    let mut run_remove = false;
    
    for file in files {
        if file.added != file.to_add {
            if file.to_add {
                run_add = true;
                add.arg(file.path);
            } else {
                run_remove = true;
                remove.arg(file.path);
            }
        }
    }

    if run_add {
        add.output()
           .expect("Failed to add files");
    }

    if run_remove {
        remove.output()
              .expect("Failed to add files");
    }
}
