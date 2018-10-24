use chargraph::CharGraph;
use charhistogram::CharHistogram;
use board::{Board, Mask, Word};


struct State<'a> {
    word : Word,
    board : &'a Board,
    lengths : &'a Vec<usize>,
    mask : Mask,
    graph : &'a CharGraph,
    dictionary_graph : &'a CharGraph,
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

fn contains_a_length(graph: &CharGraph, lengths: &Vec<usize>) -> bool {
    for n in lengths {
        if graph.contains_length(*n) {
            return true;
        }
    }
    false
}

fn walk(i: usize, j: usize, state: &State) -> Option<Vec<Vec<Word>>> {
    let currentchar = state.board.get(i, j);
    if !state.graph.subgraphs.contains_key(&currentchar) || !contains_a_length(state.graph, state.lengths) {
        return None;
    }
    let word = state.word.add(i, j, currentchar);
    let graph : &CharGraph = state.graph.subgraphs.get(&currentchar).unwrap();
    let mut solutions : Vec<Vec<Word>> = vec![];
    if graph.isword && state.lengths.contains(&word.len()) {
        if state.lengths.len()==1 {
            return Some(vec![vec![word]]);
        }
        let board = state.board.reduce(&word);
        let lengths = reduce_lengths(state.lengths, word.chars.len());
        let mask = Mask::from_board(&board);

        let nextstate = State {
                    word: Word::new(), board: &board, lengths: &lengths,
                    mask, graph: &state.dictionary_graph, ..*state };

        for j in 0..board.size() {
            for i in 0..board.rows(j) {
                match walk(i, j, &nextstate) {
                    Some(subsolutions) =>
                        for s in subsolutions {
                            let mut solution = vec![word.clone()];
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
    let nextstate = State { word, mask, graph, ..*state };
    for (i_next, j_next) in state.mask.neighbours(i, j) {
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
    let state = State {
                    word, board: &board, lengths: &lengths,
                    mask, graph: &graph, dictionary_graph: &graph};
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
