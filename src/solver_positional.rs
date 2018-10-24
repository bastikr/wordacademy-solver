use chargraph::CharGraph;
use charhistogram::CharHistogram;
use board::{Board, Mask, Word};


struct State<'a> {
    size : usize,
    word : Word,
    board : &'a Board,
    lengths : &'a Vec<usize>,
    mask : &'a Mask,
    graph : &'a CharGraph,
    dictionary : &'a Vec<&'a String>,
}

fn reduce_lengths(lengths: &Vec<usize>, l: usize) -> Vec<usize> {
    let mut removed = false;
    let mut w : Vec<usize> = Vec::with_capacity(lengths.len()-1);
    for x in lengths {
        if *x==l && !removed {
            removed = true;
            continue;
        }
        w.push(*x);
    }
    w
}

pub struct GroupedSolution {
    solutions : Vec<Vec<Word>>,
}

impl GroupedSolution {
    pub fn words(&self) -> Vec<String> {
        let v : &Vec<Word> = self.solutions.iter().next().unwrap();
        v.iter().map(|x| x.as_string()).collect()
    }

    pub fn matches(&self, solution: &Vec<Word>) -> bool {
        // let mut words = self.words();
        let words = self.words();
        for w in solution.iter() {
            let wstring = w.as_string();
            if !words.contains(&wstring) {
                return false;
            }
            // words.remove_item(&wstring);
        }
        true
    }

    pub fn push(&mut self, solution: Vec<Word>) {
        self.solutions.push(solution)
    }
}

pub fn group_solutions(solutions: Vec<Vec<Word>>) -> Vec<GroupedSolution> {
    let mut g : Vec<GroupedSolution> = vec![];
    for solution in solutions {
        let mut wasused = false;
        for gsolution in g.iter_mut() {
            if gsolution.matches(&solution) {
                gsolution.push(solution.clone());
                wasused = true;
                break;
            }
        }
        if !wasused {
            g.push(GroupedSolution {solutions: vec![solution]});
        }
    }
    g
}

fn walk(i: usize, j: usize, state: &State) -> Option<Vec<Vec<Word>>> {
    let currentchar = state.board.get(i, j);
    // println!("{}, {}: {}", i, j, state.word.add(i, j, currentchar).as_string());
    if !state.graph.subgraphs.contains_key(&currentchar) {
        return None;
    }
    let currentword = state.word.add(i, j, currentchar);
    let currentgraph : &CharGraph = state.graph.subgraphs.get(&currentchar).unwrap();
    let mut solutions : Vec<Vec<Word>> = vec![];
    if currentgraph.isword {
        if state.lengths.len()==1 {
            return Some(vec![vec![currentword]]);
        }
        let reduced_board = state.board.reduce(&currentword);
        let reduced_lengths = reduce_lengths(state.lengths, currentword.chars.len());
        let reduced_mask = Mask::from_board(&reduced_board);
        let board_histogram = CharHistogram::from_board(&reduced_board);
        let reduced_words : Vec<&String> = state.dictionary.into_iter().filter(|x| reduced_lengths.contains(&x.len()))
                            .filter(|x| board_histogram.writeable(x)).map(|x| *x)
                            .collect();
        let reduced_graph = CharGraph::from_strings(&reduced_words);
        let reduced_state = State {size: state.size,
                    word: Word::new(), board: &reduced_board, lengths: &reduced_lengths,
                    mask: &reduced_mask, graph: &reduced_graph, dictionary: &reduced_words };
        // println!("Found word: {}", currentword.as_string());
        // println!("Reduced words: {}", reduced_words.len());
        // println!("Reduced board: {:?}", reduced_board);
        // println!("Reduced mask: {:?}", reduced_mask);
        for j in 0..reduced_board.size() {
            for i in 0..reduced_board.rows(j) {
                match walk(i, j, &reduced_state) {
                    Some(subsolutions) =>
                // println!("Found {} subsolutions.", subsolutions.len());
                for s in subsolutions {
                    let mut solution = vec![currentword.clone()];
                    solution.extend(s);
                    solutions.push(solution);
                        },
                    None => {}
                }
            }
        }
    }
    let mut mask = state.mask.clone();
    mask.set(i, j, false);
    let nextstate = State {size: state.size,
                word: currentword, board: state.board, lengths: state.lengths,
                mask: &mask, graph: currentgraph, dictionary: state.dictionary };
    for (i_next, j_next) in mask.neighbours(i, j) {
        match walk(i_next, j_next, &nextstate) {
            Some(subsolutions) => solutions.extend(subsolutions),
            None => {}
        }
    }
    return Some(solutions);
}

pub fn solve<'a>(boardstring: &str, lengths: Vec<usize>, words: &[&'a String]) -> Vec<Vec<Word>> {
    let board = Board::from_string(boardstring);
    let size = board.size();
    let board_histogram = CharHistogram::from_board(&board);
    let reduced_words : Vec<&'a String> = words.into_iter().filter(|x| lengths.contains(&x.len()))
                            .filter(|x| board_histogram.writeable(x)).map(|x| *x)
                            .collect();
    let graph = CharGraph::from_strings(&reduced_words);

    let mut solutions : Vec<Vec<Word>> = vec![];
    let mask = Mask::new(size);
    let word = Word::new();
    let state = State {size: size,
                    word: word, board: &board, lengths: &lengths,
                    mask: &mask, graph: &graph, dictionary: &reduced_words };
    for i in 0..size {
        for j in 0..size {
            match walk(i, j, &state) {
                Some(subsolutions) => solutions.extend(subsolutions),
                None => {}
            }
        }
    }
    solutions
}