use std::os::unix::net::UnixListener;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub enum Action {
    Exit,
    Exec(String),
    Flip
}

pub struct ActionSocket {
    listener: UnixListener
}

impl ActionSocket {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let _ = std::fs::remove_file(&path);
        let listener = UnixListener::bind(path)?;
        listener.set_nonblocking(true)?;
        Ok(Self {listener})
    }

    pub fn pool(&self) -> Vec<Action> {
        let mut actions = vec![];

        if let Ok((stream, _)) = self.listener.accept() {
            let reader = BufReader::new(stream);
            for line in reader.lines().flatten() {
                if let Some(action) = parse_action(&line) {
                    actions.push(action);
                }
            }
        }

        actions
    }
}

fn parse_action(input: &str) -> Option<Action> {
    let (token, rest) = {
        if let Some(idx) = input.find(char::is_whitespace) {
            let after = input[idx..].trim_start();
            (&input[..idx], after)
        } else {
            (input, "")
        }
    };
    println!("{}",token);
    match token {
        "EXIT" => Some(Action::Exit),
        "EXEC" => Some(Action::Exec(rest.to_string())),
        "FLIP" => Some(Action::Flip),
        _ => None
    }
}