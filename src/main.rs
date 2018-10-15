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
            Some(count_available) => count>count_available,
            None => true
        } {return false;}
    }
    return true;
}

fn writeable_with(word: &str, boardletters: &HashMap<char, u8>) -> bool {
    let wordhistogram = histogram(word);
    contains(&wordhistogram, &boardletters)
}

struct Board<'a> {
    words : Vec<&'a str>,
    wordlengths : Vec<u8>,
    columns : Vec<String>,
    letters : HashMap<char, u8>,
}

fn create_board<'a>(board: &str, wordlengths: Vec<u8>, words: Vec<&'a str>) -> Board<'a> {
    let board_histogram = histogram(board);
    let selection : Vec<&'a str> = words.into_iter().filter(|x| wordlengths.contains(&(x.len() as u8)))
                                .filter(|x| writeable_with(x, &board_histogram))
                                .collect();
    // let columns : Vec<String> = vec![ "uceg".to_string(), "cpbh".to_string(), "eoca".to_string(), "gcil".to_string()];
    let mut columns = vec![];
    let chars : Vec<char> = board.chars().collect();
    for j in 0..4 {
        let mut column = String::with_capacity(4);
        for i in 0..4 {
            column.push(chars[i*4 + j]);
        }
        columns.push(column);
    }

    // columns.push("uceg".to_string());
    Board { words: selection, wordlengths: wordlengths, columns: columns, letters: board_histogram }
}

struct Solution<'a> {
    words: Vec<&'a str>,
    i: Vec<Vec<u8>>,
    j: Vec<Vec<u8>>,
}

struct Mask {
    data : Vec<Vec<bool>>,
}

fn create_mask(board: &Board) -> Mask {
    let mut mask : Vec<Vec<bool>> = vec![];
    for column in &board.columns {
        mask.push(vec![true; column.len()]);
    }
    Mask {data : mask}
}

fn reduced_histogram(hist: &HashMap<char, u8>, letters: &str) -> HashMap<char, u8> {
    let letter_hist = histogram(letters);
    let mut h : HashMap<char, u8> = HashMap::new();
    for (letter, count) in hist.iter() {
        let c = match(letter_hist.get(letter)) {
            Some(letters_count) => *letters_count,
            None => 0
        };
        if count - c > 0 {
            h.insert(*letter, count-c);
        }
    }
    h
}

fn reduced_wordlengths(wordlengths: &Vec<u8>, l: u8) -> Vec<u8> {
    let removed = false;
    let mut w : Vec<u8> = Vec::with_capacity(wordlengths.len()-1);
    for x in wordlengths {
        if *x==l && !removed {
            continue;
        }
        w.push(*x);
    }
    w
}

fn arbitrary_solutions<'a>(wordlengths: Vec<u8>, words: Vec<&'a str>, letters : HashMap<char, u8>) -> Vec<Vec<&'a str>> {
    let reduced_words : Vec<&'a str> = words.into_iter().filter(|x| wordlengths.contains(&(x.len() as u8)))
                            .filter(|x| writeable_with(x, &letters))
                            .collect();
    // println!("{}: {}", wordlengths.len(), reduced_words.len());
    let mut solutions = Vec::new();
    if wordlengths.len()==1 {
        for word in reduced_words {
            solutions.push(vec![word]);
        }
        return solutions;
    }
    for word in reduced_words.clone() {
        let reduced_letters = reduced_histogram(&letters, &word);
        let reduced_wordlengths = reduced_wordlengths(&wordlengths, word.len() as u8);
        let subsolutions = arbitrary_solutions(reduced_wordlengths, reduced_words.clone(), reduced_letters);
        for mut sol in subsolutions {
            sol.push(word);
            solutions.push(sol);
        }
    }
    solutions
}

// fn solutions(board: &Board, words: &Vec<&str>) -> Vec<Solution> {

// }

fn main() {
    // let mut f = File::open("data/words.txt").expect("file not found");
    let mut f = File::open("/usr/share/dict/cracklib-small").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let words_lower = contents.split("\n").map(|x| x.to_string().to_lowercase()).collect::<Vec<String>>();
    let words = words_lower.iter().map(|x| &x[..]).collect::<Vec<&str>>();
    println!("Number of loaded words {}", words.len());

    // let board = create_board("ucegtpocebcilhal", vec![7, 6, 3], words);
    // for column in board.columns {
        // println!("{}", column);
    // }

    let board = "ucegtpocebcilhal";
    // let board = "horuaste";
    let board_histogram = histogram(board);
    // let wordlengths = vec![5, 3];
    let wordlengths = vec![7, 6, 3];
    let S = arbitrary_solutions(wordlengths, words, board_histogram);
    for solution in S.iter() {
        for word in solution {
            print!("{} ", word);
        }
        println!("");
    }
    println!("{} solutions found", S.len());

    // let word_histogram = histogram("rat");
    // let reducedboard_histogram = reduced_histogram(&board_histogram, "rat");

    // for n in 1..40 {
    //     let selection = words.iter().filter(|x| x.len()==n)
    //                                 .filter(|x| writeable_with(x, &board_histogram))
    //                                 .collect::<Vec<&&str>>();
    //     println!("{}: {}", n, selection.len());
    // }

    // for (key, value) in reducedboard_histogram.iter() {
    //     println!("{}: {}", key, value);
    // }

    // for word in words.iter().filter(|x| x.len()==5).filter(|x| writeable_with(x, &board_histogram)) {
    //     println!("{}", word);
    // }
    // for word in words.iter().filter(|x| x.len()==5).filter(|x| **x=="house") {
    //     println!("{}", word);
    // }
}
