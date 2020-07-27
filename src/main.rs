use std::process::Command;
use std::str;

fn main() {
    struct File {
        path: String,
        selected: bool,
    }

    parse_status();
}

fn parse_status() {
    let result = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output()
            .expect("failed to run git status");
    
    for item in result.stdout.split(|num| num == &10) {
        let info = item.split_at(2);
        
        println!("{:?}", item);
    }
}
