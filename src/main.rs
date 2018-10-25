extern crate wordacademy_solver;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut iter = args.into_iter();
    iter.next().expect("Should have been the name of the program.");
    let board = iter.next().expect("Expected board as first argument.");
    let size = (board.len() as f64).sqrt() as usize;
    if size*size!=board.len() {
        panic!("Board length does not fit into a square.");
    }

    let mut wordlengths : Vec<usize> = vec![];
    for x in iter {
        wordlengths.push(x.parse::<usize>().expect("Error parsing wordlength."));
    }
    let totallength : usize  = wordlengths.iter().sum();
    if totallength!=board.len() {
        panic!("Word lengths don't add up to size of the board.");
    }

    let board_ = wordacademy_solver::board::Board::from_string(&board);
    println!("Solve board: {}", board_);

    // let words_lower = wordacademy_solver::dictionary::load("/usr/share/dict/cracklib-small");
    // let words_lower = wordacademy_solver::dictionary::load("data/words.txt");
    let words_lower = wordacademy_solver::dictionary::load("data/word_list_german_spell_checked.txt");
    let words = words_lower.iter().collect::<Vec<&String>>();
    println!("Number of loaded words {}", words.len());

    // let board = "ucegtpocebcilhal";
    // let wordlengths = vec![7, 6, 3];
    // let board = "breaeetfarhskcet";
    // let wordlengths = vec![7, 4, 5];
    // let board = "rhilcodiyialsdac";
    // let wordlengths = vec![5, 5, 6];
    // let board = "ildcnlotgouoerrs";
    // let wordlengths = vec![6, 7, 3];
    
    let solutions = wordacademy_solver::solver_positional::solve(&board, &wordlengths, &words);
    let gsolutions = wordacademy_solver::solver_positional::group_solutions(solutions);
    println!("Found {} solutions.", gsolutions.len());
    for solution in gsolutions {
        println!("{:?}", solution.words());
    }
}
