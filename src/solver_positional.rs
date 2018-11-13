use board::{Board, Mask};
use word::Word;
use chargraph::CharGraph;
use charhistogram::CharHistogram;

struct State<'a> {
    word: Word,
    previous_word: Option<&'a Word>,
    board: &'a Board,
    lengths: &'a Vec<usize>,
    mask: Mask,
    graph: &'a CharGraph,
    dictionary_graph: &'a CharGraph,
    dictionary: &'a Vec<String>,
}

fn reduce_lengths(lengths: &[usize], l: usize) -> Vec<usize> {
    let mut removed = false;
    let mut w: Vec<usize> = Vec::with_capacity(lengths.len() - 1);
    for x in lengths {
        if *x == l && !removed {
            removed = true;
            continue;
        }
        w.push(*x);
    }
    w
}

pub struct GroupedSolution {
    solutions: Vec<Vec<Word>>,
}

impl GroupedSolution {
    pub fn words(&self) -> Vec<String> {
        let v: &Vec<Word> = self.solutions.iter().next().unwrap();
        v.iter().map(|x| x.as_string()).collect()
    }

    pub fn len(&self) -> usize {
        self.solutions.len()
    }

    pub fn matches(&self, solution: &[Word]) -> bool {
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
    let mut g: Vec<GroupedSolution> = vec![];
    for solution in solutions {
        let mut wasused = false;
        for gsolution in &mut g {
            if gsolution.matches(&solution) {
                gsolution.push(solution.clone());
                wasused = true;
                break;
            }
        }
        if !wasused {
            g.push(GroupedSolution {
                solutions: vec![solution],
            });
        }
    }
    g
}

fn contains_a_length(graph: &CharGraph, lengths: &[usize]) -> bool {
    for n in lengths {
        if graph.contains_length(*n) {
            return true;
        }
    }
    false
}

fn walk(i: usize, j: usize, state: &State) -> Option<Vec<Vec<Word>>> {
    let currentchar = state.board.get(i, j);
    if !state.graph.contains_key(&currentchar)
        || !contains_a_length(state.graph, state.lengths)
    {
        return None;
    }
    let word = state.word.add(i, j, currentchar);
    let graph: &CharGraph = &state.graph.subgraph(&currentchar);
    let mut solutions: Vec<Vec<Word>> = vec![];
    if graph.isword() && state.lengths.contains(&word.len()) {
        if state.lengths.len() == 1 {
            return Some(vec![vec![word]]);
        }
        let is_permutation = match state.previous_word {
            Some(w) => {
                word.is_before_previous(w) && word.commutates_with_previous(w)
            },
            None => false
        };
        if !is_permutation {
            if state.lengths.len() > 6 {
                println!("{}", word.as_string());
            }
            let board = state.board.reduce(&word);
            let lengths = reduce_lengths(state.lengths, word.chars.len());
            let mask = Mask::from_board(&board);
            if state.lengths.len() > 4 {
                let dictionary = reduce_words(&board, &lengths, state.dictionary);
                let graph = CharGraph::from_strings(&dictionary);

                let nextstate = State {
                    word: Word::new(),
                    previous_word: Some(&state.word),
                    board: &board,
                    lengths: &lengths,
                    mask,
                    graph: &graph,
                    dictionary_graph: &graph,
                    dictionary: &dictionary,
                    ..*state
                };

                for j in 0..board.size() {
                    for i in 0..board.rows(j) {
                        if let Some(subsolutions) = walk(i, j, &nextstate) {
                            for s in subsolutions {
                                let mut solution = vec![word.clone()];
                                solution.extend(s);
                                solutions.push(solution);
                            }
                        }
                    }
                }
            } else {
                let nextstate = State {
                    word: Word::new(),
                    previous_word: Some(&state.word),
                    board: &board,
                    lengths: &lengths,
                    mask,
                    graph: &state.dictionary_graph,
                    ..*state
                };

                for j in 0..board.size() {
                    for i in 0..board.rows(j) {
                        if let Some(subsolutions) = walk(i, j, &nextstate) {
                            for s in subsolutions {
                                let mut solution = vec![word.clone()];
                                solution.extend(s);
                                solutions.push(solution);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut mask = state.mask.clone();
    mask.set(i, j, false);
    let nextstate = State {
        word,
        mask,
        graph,
        ..*state
    };
    // for (i_next, j_next) in state.mask.neighbours(i, j).filter(|x| state.mask.get(x.0, x.1)) {
    // // for (i_next, j_next) in state.mask.neighbours2(i, j) {
    //     if let Some(subsolutions) = walk(i_next, j_next, &nextstate) {
    //         solutions.extend(subsolutions)
    //     }
    // }
    let size = state.board.size();
    // if i==0 {
    //     if j==0 {
    //         for (i_next, j_next) in [(0, 1), (1, 0), (1, 1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //             if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //                 solutions.extend(subsolutions)
    //             }
    //         }
    //     } else if j+1==size{
    //         for (i_next, j_next) in [(0, j-1), (1, j-1), (1, j)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //             if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //                 solutions.extend(subsolutions)
    //             }
    //         }
    //     } else {
    //         for (i_next, j_next) in [(0, j-1), (0, j+1), (1, j-1), (1, j), (1, j+1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //             if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //                 solutions.extend(subsolutions)
    //             }
    //         }
    //     }
    // } else if i+1==size {
    //     if j==0 {
    //         for (i_next, j_next) in [(i-1, 0), (i-1, 1), (i, 1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //             if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //                 solutions.extend(subsolutions)
    //             }
    //         }
    //     } else if j+1==size{
    //         for (i_next, j_next) in [(i-1, j-1), (i-1, j), (i, j-1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //             if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //                 solutions.extend(subsolutions)
    //             }
    //         }
    //     } else {
    //         for (i_next, j_next) in [(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //             if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //                 solutions.extend(subsolutions)
    //             }
    //         }
    //     }
    // } else if j==0 {
    //     for (i_next, j_next) in [(i-1, j), (i-1, j+1), (i, j+1), (i+1, j), (i+1, j+1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //         if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //             solutions.extend(subsolutions)
    //         }
    //     }
    // } else if j+1==size {
    //     for (i_next, j_next) in [(i-1, j-1), (i-1, j), (i, j-1), (i+1, j-1), (i+1, j)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //         if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //             solutions.extend(subsolutions)
    //         }
    //     }
    // } else {
    //     for (i_next, j_next) in [(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1), (i+1, j-1), (i+1, j), (i+1, j+1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
    //         if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
    //             solutions.extend(subsolutions)
    //         }
    //     }
    // }
    if i==0 {
        if j==0 {
            if state.mask.get(0, 1) {
                if let Some(subsolutions) = walk(0, 1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(1, 0) {
                if let Some(subsolutions) = walk(1, 0, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(1, 1) {
                if let Some(subsolutions) = walk(1, 1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            // for (i_next, j_next) in [(0, 1), (1, 0), (1, 1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
            //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
            //         solutions.extend(subsolutions)
            //     }
            // }
        } else if j+1==size {
            if state.mask.get(0, j-1) {
                if let Some(subsolutions) = walk(0, j-1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(1, j-1) {
                if let Some(subsolutions) = walk(1, j-1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(1, j) {
                if let Some(subsolutions) = walk(1, j, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            // for (i_next, j_next) in [(0, j-1), (1, j-1), (1, j)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
            //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
            //         solutions.extend(subsolutions)
            //     }
            // }
        } else {
            if state.mask.get(0, j-1) {
                if let Some(subsolutions) = walk(0, j-1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(0, j+1) {
                if let Some(subsolutions) = walk(0, j+1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(1, j-1) {
                if let Some(subsolutions) = walk(1, j-1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(1, j) {
                if let Some(subsolutions) = walk(1, j, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(1, j+1) {
                if let Some(subsolutions) = walk(1, j+1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            // for (i_next, j_next) in [(0, j-1), (0, j+1), (1, j-1), (1, j), (1, j+1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
            //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
            //         solutions.extend(subsolutions)
            //     }
            // }
        }
    } else if i+1==size {
        if j==0 {
            if state.mask.get(i-1, 0) {
                if let Some(subsolutions) = walk(i-1, 0, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(i-1, 1) {
                if let Some(subsolutions) = walk(i-1, 1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(i, 1) {
                if let Some(subsolutions) = walk(i, 1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            // for (i_next, j_next) in [(i-1, 0), (i-1, 1), (i, 1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
            //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
            //         solutions.extend(subsolutions)
            //     }
            // }
        } else if j+1==size{
            if state.mask.get(i-1, j-1) {
                if let Some(subsolutions) = walk(i-1, j-1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(i-1, j) {
                if let Some(subsolutions) = walk(i-1, j, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(i, j-1) {
                if let Some(subsolutions) = walk(i, j-1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            // for (i_next, j_next) in [(i-1, j-1), (i-1, j), (i, j-1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
            //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
            //         solutions.extend(subsolutions)
            //     }
            // }
        } else {
            if state.mask.get(i-1, j-1) {
                if let Some(subsolutions) = walk(i-1, j-1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(i-1, j) {
                if let Some(subsolutions) = walk(i-1, j, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(i-1, j+1) {
                if let Some(subsolutions) = walk(i-1, j+1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(i, j-1) {
                if let Some(subsolutions) = walk(i, j-1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            if state.mask.get(i, j+1) {
                if let Some(subsolutions) = walk(i, j+1, &nextstate) {
                    solutions.extend(subsolutions)
                }
            }
            // for (i_next, j_next) in [(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
            //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
            //         solutions.extend(subsolutions)
            //     }
            // }
        }
    } else if j==0 {
        if state.mask.get(i-1, j) {
            if let Some(subsolutions) = walk(i-1, j, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i-1, j+1) {
            if let Some(subsolutions) = walk(i-1, j+1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i, j+1) {
            if let Some(subsolutions) = walk(i, j+1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i+1, j) {
            if let Some(subsolutions) = walk(i+1, j, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i+1, j+1) {
            if let Some(subsolutions) = walk(i+1, j+1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        // for (i_next, j_next) in [(i-1, j), (i-1, j+1), (i, j+1), (i+1, j), (i+1, j+1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
        //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
        //         solutions.extend(subsolutions)
        //     }
        // }
    } else if j+1==size {
        if state.mask.get(i-1, j-1) {
            if let Some(subsolutions) = walk(i-1, j-1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
         if state.mask.get(i-1, j) {
            if let Some(subsolutions) = walk(i-1, j, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
         if state.mask.get(i, j-1) {
            if let Some(subsolutions) = walk(i, j-1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
         if state.mask.get(i+1, j-1) {
            if let Some(subsolutions) = walk(i+1, j-1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
         if state.mask.get(i+1, j) {
            if let Some(subsolutions) = walk(i+1, j, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        // for (i_next, j_next) in [(i-1, j-1), (i-1, j), (i, j-1), (i+1, j-1), (i+1, j)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
        //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
        //         solutions.extend(subsolutions)
        //     }
        // }
    } else {
        if state.mask.get(i-1, j-1) {
            if let Some(subsolutions) = walk(i-1, j-1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i-1, j) {
            if let Some(subsolutions) = walk(i-1, j, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i-1, j+1) {
            if let Some(subsolutions) = walk(i-1, j+1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i, j-1) {
            if let Some(subsolutions) = walk(i, j-1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i, j+1) {
            if let Some(subsolutions) = walk(i, j+1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i+1, j-1) {
            if let Some(subsolutions) = walk(i+1, j-1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i+1, j) {
            if let Some(subsolutions) = walk(i+1, j, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        if state.mask.get(i+1, j+1) {
            if let Some(subsolutions) = walk(i+1, j+1, &nextstate) {
                solutions.extend(subsolutions)
            }
        }
        // for (i_next, j_next) in [(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1), (i+1, j-1), (i+1, j), (i+1, j+1)].into_iter().filter(|x| state.mask.get(x.0, x.1)) {
        //     if let Some(subsolutions) = walk(*i_next, *j_next, &nextstate) {
        //         solutions.extend(subsolutions)
        //     }
        // }
    }
    Some(solutions)
}

fn column_contains_char(j: usize, board: &Board, x: char) -> bool {
    for i in 0..board.size() {
        if board.get(i, j) == x {
            return true;
        }
    }
    false
}

fn writeable(word: &str, board: &Board) -> bool {
    let mut it = word.chars();
    let mut x0 = it.next().unwrap();
    for x1 in it {
        let mut found_pair = false;
        for j in 0..board.size() {
            if !column_contains_char(j, board, x0) {
                continue;
            }
            if j>0 && column_contains_char(j-1, board, x1) {
                found_pair = true;
                break;
            }
            if column_contains_char(j, board, x1) {
                found_pair = true;
                break;
            }
            if j+1<board.size() && column_contains_char(j+1, board, x1) {
                found_pair = true;
                break;
            }
        }
        if !found_pair {
            return false;
        }
        x0 = x1;
    }
    true
}

fn writeable2(word: &str, board: &Board) -> bool {
    let mut v0 = vec![true; board.size()];
    let mut v1 = vec![false; board.size()];
    for x in word.chars() {
        for j in 0..board.size() {
            if v0[j] && column_contains_char(j, board, x) {
                if j>0 {
                    v1[j-1] = true;
                }
                v1[j] = true;
                if j+1<board.size() {
                    v1[j+1] = true;
                }
            }
        }
        if !v1.iter().any(|x| *x) {
            return false;
        }
        let tmp = v0;
        v0 = v1;
        v1 = tmp;
        // v0.swap(v1);
        v1.iter_mut().map(|x| *x = false).count();
    }
    return true;
}

pub fn reduce_words(board: &Board, lengths: &[usize], words: &[String]) -> Vec<String> {
    let board_histogram = CharHistogram::from_board(board);
    let reduced_words: Vec<String> = words
        .into_iter()
        .filter(|x| lengths.contains(&x.len()))
        .filter(|x| board_histogram.writeable(x))
        .filter(|x| writeable2(x, board))
        .cloned()
        .collect();
    reduced_words
}

pub fn solve(boardstring: &str, lengths: &[usize], words: &[String]) -> Vec<Vec<Word>> {
    let board = Board::from_string(boardstring);
    let size = board.size();
    let reduced_words = reduce_words(&board, lengths, words);
    let graph = CharGraph::from_strings(&reduced_words);
    println!("Number of reduced words: {}", reduced_words.len());
    let mut solutions: Vec<Vec<Word>> = vec![];
    let mask = Mask::new(size);
    let word = Word::new();
    let lengths_: Vec<usize> = Vec::from(lengths);
    let state = State {
        word,
        previous_word: None,
        board: &board,
        lengths: &lengths_,
        mask,
        graph: &graph,
        dictionary_graph: &graph,
        dictionary: &reduced_words
    };
    for i in 0..size {
        for j in 0..size {
            // let i = 2;
            // let j = 2;
            println!("i={} j={}", i, j);
            if let Some(subsolutions) = walk(i, j, &state) {
                solutions.extend(subsolutions)
            }
        }
    }
    solutions
}
