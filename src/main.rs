extern crate wordacademy_solver;


fn main() {
    let words_lower = wordacademy_solver::dictionary::load("/usr/share/dict/cracklib-small");
    // let words_lower = wordacademy_solver::dictionary::load("data/words.txt");
    let words = words_lower.iter().collect::<Vec<&String>>();
    println!("Number of loaded words {}", words.len());

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
        println!("{:?}", solution.words());
    }
}
