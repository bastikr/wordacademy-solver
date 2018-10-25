use std::fs::File;
use std::io::prelude::*;

pub fn load(filename: &str) -> Vec<String> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
        .split('\n')
        .map(|x| x.to_string().to_lowercase())
        .collect::<Vec<String>>()
}

fn format_hunspellword(w: &str) -> String {
    w.split('/').next().unwrap().to_lowercase()
}

pub fn load_hunspell(filename: &str) -> Vec<String> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
        .split('\n')
        .skip(15)
        .map(|x| format_hunspellword(x))
        .collect::<Vec<String>>()
}
