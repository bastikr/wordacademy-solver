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

fn walk(i: usize, j: usize, state: &State) -> Option<Vec<Vec<Word>>> {
    let currentchar = state.board.get(i, j);
    if !state.graph.subgraphs.contains_key(&currentchar) {
        return None;
    }
    let currentword = state.word.add(i, j, currentchar);
    let currentgraph : &CharGraph = state.graph.subgraphs.get(&currentchar).unwrap();
    let mut solutions : Vec<Vec<Word>> = vec![];
    if currentgraph.isword && state.lengths.contains(&currentword.len()) {
        if state.lengths.len()==1 {
            return Some(vec![vec![currentword]]);
        }
        let reduced_board = state.board.reduce(&currentword);
        let reduced_lengths = reduce_lengths(state.lengths, currentword.chars.len());
        let reduced_mask = Mask::from_board(&reduced_board);

        let reduced_state = State {
                    word: Word::new(), board: &reduced_board, lengths: &reduced_lengths,
                    mask: reduced_mask, graph: &state.dictionary_graph, ..*state };

        for j in 0..reduced_board.size() {
            for i in 0..reduced_board.rows(j) {
                match walk(i, j, &reduced_state) {
                    Some(subsolutions) =>
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
    let nextstate = State {
                word: currentword,
                mask: mask, graph: currentgraph,
                ..*state };
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
                    word: word, board: &board, lengths: &lengths,
                    mask: mask, graph: &graph, dictionary_graph: &graph};
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