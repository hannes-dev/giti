use std::io::{self, stdin, stdout, Write};
use std::process::Command;
use std::str;
use termion::color;
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use git2::Repository;

struct File {
    path: String,
    added: bool,
    to_add: bool,
}

fn main() {
    let files = parse_status();
    if files.is_empty() {
        println!("No files have been changed.")
    } else {
        run_interface(files).unwrap();
    }
}

fn parse_status() -> Vec<File> {
    let repo = match Repository::discover("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("{:?}", repo.statuses(None).unwrap().get(0).unwrap().status());

    if !error.is_empty() {
        println!("Git gave this error: {}", error);

        return vec![];
    }

    // Empty output, no changes
    if output.trim_start().is_empty() {
        return vec![];
    }

    output
        .lines()
        .map(|line| {
            let (status, path) = line.split_at(3);
            let added = !status.starts_with(' ')
                && !status.starts_with('?')
                && status.chars().nth(1).unwrap() != 'M';

            File {
                path: path.to_owned(),
                added,
                to_add: added,
            }
        })
        .collect()
}

fn print_status(files: &[File], selected: usize) {
    // loop over files, adding x and making them green if they are (going to be) staged
    for (index, file) in files.iter().enumerate() {
        let (check, color) = if file.to_add {
            ("x", format!("{}", color::Fg(color::Green)))
        } else {
            (" ", format!("{}", color::Fg(color::Red)))
        };

        // set select marks around the currently selected file
        let select_marks = if index == selected {
            [">", "<"]
        } else {
            [" ", " "]
        };

        // print line with file info. example of a selected and added file: ">[x] .gitignore<"
        println!(
            "{}{}[{}] {}{}{}\r",
            select_marks[0],
            color,
            check,
            file.path,
            color::Fg(color::Reset),
            select_marks[1]
        );
    }
}

fn run_interface(mut files: Vec<File>) -> Result<(), io::Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    let mut selected = 0;

    println!("Use arrow keys to select, space to toggle and enter to confirm. Esc or q to quit.\r");
    print_status(&files, selected);

    // listen for key-presses
    for c in stdin.keys() {
        match c? {
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Up => {
                if selected > 0 {
                    selected -= 1
                }
            }
            Key::Down => {
                if selected < files.len() - 1 {
                    selected += 1
                }
            }
            Key::Char(' ') => files[selected].to_add ^= true,
            Key::Char('\n') => {
                commit_changes(files);
                break;
            }
            _ => (),
        }

        // get current cursor position
        let position = stdout.cursor_pos()?.1;
        // clear lines equal to the amount of files, starting from 1 above the cursor.
        for number in 1..files.len() + 1 {
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, position - number as u16),
                termion::clear::CurrentLine
            )?;
        }
        print_status(&files, selected);
        stdout.flush()?;
    }
    Ok(())
}

fn commit_changes(files: Vec<File>) {
    let mut add = Command::new("git");
    add.arg("add");

    let mut remove = Command::new("git");
    remove.arg("restore").arg("--staged");

    let mut add_amount = 0;
    let mut remove_amount = 0;

    for file in files {
        // check if file status actually changed
        if file.added != file.to_add {
            if file.to_add {
                add_amount += 1;
                add.arg(file.path);
            } else {
                remove_amount += 1;
                remove.arg(file.path);
            }
        }
    }

    if add_amount > 0 {
        add.output().expect("Failed to add files");
    }

    if remove_amount > 0 {
        remove.output().expect("Failed to add files");
    }

    println!(
        "You staged {} and unstaged {} file(s).\r",
        add_amount, remove_amount
    );
}
