extern crate wordacademy_solver;

use std::collections::HashSet;
// fn solutions(board: &Board, words: &Vec<&str>) -> Vec<Solution> {

// }

fn main() {
    // let mut f = File::open("data/words.txt").expect("file not found");
    // let mut f = File::open("/usr/share/dict/cracklib-small").expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");
    // let words_lower = contents.split("\n").map(|x| x.to_string().to_lowercase()).collect::<Vec<String>>();
    let words_lower = wordacademy_solver::dictionary::load("/usr/share/dict/cracklib-small");
    let words = words_lower.iter().collect::<Vec<&String>>();
    println!("Number of loaded words {}", words.len());

    // let board = create_board("ucegtpocebcilhal", vec![7, 6, 3], words);
    // for column in board.columns {
        // println!("{}", column);
    // }

    // let board = "ucegtpocebcilhal";
    // let wordlengths = vec![7, 6, 3];
    // let board = "breaeetfarhskcet";
    // let wordlengths = vec![7, 4, 5];
    // let board = "rhilcodiyialsdac";
    // let wordlengths = vec![5, 5, 6];
    let board = "ildcnlotgouoerrs";
    let wordlengths = vec![6, 7, 3];
    
    let solutions = wordacademy_solver::solver_positional::solve(board, wordlengths, &words);
    let gsolutions = wordacademy_solver::solver_positional::group_solutions(solutions);
    println!("Found {} solutions.", gsolutions.len());
    for solution in gsolutions {
        // let strings : Vec<String> = solution.iter().map(|x| x.as_string()).collect();
        println!("{:?}", solution.words());
    }
    // println!("Found {:?} solutions.", solutions.iter().map(|x| x.iter().map(|w| w.)));
    // let board = "horuaste";
    // let board_histogram = histogram(board);
    // let wordlengths = vec![5, 3];
    // let wordlengths = vec![7, 6, 3];
    // let solutions = arbitrary_solutions(wordlengths, &words, board_histogram);
    // for solution in solutions.iter() {
    //     for word in solution {
    //         print!("{} ", word);
    //     }
    //     println!("");
    // }
    // println!("{} solutions found", solutions.len());

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
