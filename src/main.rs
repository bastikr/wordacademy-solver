use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn histogram(word: &str) -> HashMap<char, u8> {
    let mut h = HashMap::new();
    for letter in word.chars() {
        let count = h.entry(letter).or_insert(0);
        *count += 1;
    }
    h
}

fn contains(subhistogram: &HashMap<char, u8>, histogram: &HashMap<char, u8>) -> bool {
    for (letter, count) in subhistogram {
        if match histogram.get(&letter) {
            Some(count_available) => count<=count_available,
            None => false
        } {return false;}
    }
    return true;
}

fn writeable_with(word: &str, boardletters: &HashMap<char, u8>) -> bool {
    let wordhistogram = histogram(word);
    contains(&wordhistogram, &boardletters)
}

fn main() {
    let mut f = File::open("data/words.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let words = contents.split("\n").collect::<Vec<&str>>();
    println!("Number of loaded words {}", words.len());

    let board = "ucegtpocebcilhal";
    let board_histogram = histogram(board);
    let wordlengths = vec![7, 6, 3];

    for n in 1..40 {
        let selection = words.iter().filter(|x| x.len()==n)
                                    .filter(|x| writeable_with(x, &board_histogram))
                                    .collect::<Vec<&&str>>();
        println!("{}: {}", n, selection.len());
    }
}
