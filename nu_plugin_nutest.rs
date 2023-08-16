//use std::process::Command;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("test");
}

