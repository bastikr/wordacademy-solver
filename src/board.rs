use std::fmt;
use word::Word;


#[derive(Debug)]
pub struct Board {
    data: Vec<char>,
    rows: Vec<usize>,
}

impl Board {
    pub fn from_string(boardstring: &str) -> Board {
        let size = (boardstring.len() as f64).sqrt() as usize;
        assert!(size * size == boardstring.len());
        let chars: Vec<char> = boardstring.chars().collect();
        let mut data: Vec<char> = Vec::with_capacity(size * size);
        for j in 0..size {
            for i in (0..size).rev() {
                data.push(chars[size * i + j]);
            }
        }
        Board {
            data,
            rows: vec![size; size],
        }
    }

    pub fn size(&self) -> usize {
        self.rows.len()
    }

    pub fn rows(&self, j: usize) -> usize {
        self.rows[j]
    }

    pub fn get(&self, i: usize, j: usize) -> char {
        self.data[self.size() * j + i]
    }

    pub fn reduce(&self, word: &Word) -> Board {
        let size = self.size();
        let mut data: Vec<char> = Vec::with_capacity(self.data.len());
        let mut rows: Vec<usize> = Vec::with_capacity(size);
        for j in 0..size {
            let mut rowcounter = 0;
            for i in 0..self.rows(j) {
                if word.contains_coordinates(&(i, j)) {
                    continue;
                }
                data.push(self.get(i, j));
                rowcounter += 1;
            }
            for _i in 0..size - rowcounter {
                data.push(' ');
            }
            rows.push(rowcounter);
        }
        Board { data, rows }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut separator = "+-".repeat(self.size());
        separator.push_str("+");
        writeln!(f);
        for i in (0..self.size()).rev() {
            writeln!(f, "{}", separator);
            for j in 0..self.size() {
                write!(f, "|");
                if self.rows(j) > i {
                    write!(f, "{}", self.get(i, j));
                } else {
                    write!(f, " ");
                }
            }
            writeln!(f, "|");
        }
        writeln!(f, "{}", separator)
    }
}


pub struct Neighbours {
    count: usize,
    size: usize,
    i0: usize,
    j0: usize,
}

impl Neighbours {
    pub fn new(size: usize, i: usize, j: usize) -> Neighbours {
        Neighbours { count: 0, size: size, i0: i, j0: j}
    }
}

impl Iterator for Neighbours {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        match self.count {
            0 => {
                if self.i0==0 {
                    if self.j0+1==self.size {
                        self.count = 5;
                    } else {
                        self.count = 3;
                    }
                    return self.next();
                }
                if self.j0==0 {
                    self.count = 1;
                    return self.next();
                }
                self.count = 1;
                return Some((self.i0-1, self.j0-1));
            },
            1 => {
                if self.j0+1==self.size {
                    if self.i0+1==self.size {
                        self.count = 7;
                    } else {
                        self.count = 5;
                    }
                } else {
                    self.count = 2
                }
                return Some((self.i0-1, self.j0));
            },
            2 => {
                self.count = 3;
                return Some((self.i0-1, self.j0+1));
            },
            3 => {
                if self.i0+1==self.size {
                    if self.j0==0 {
                        self.count = 8;
                    } else {
                        self.count = 7;
                    }
                } else {
                    self.count = 4;
                }
                return Some((self.i0, self.j0+1));
            },
            4 => {
                self.count = 5;
                return Some((self.i0+1, self.j0+1));
            },
            5 => {
                if self.j0==0 {
                    self.count = 8;
                } else {
                    self.count = 6;
                }
                return Some((self.i0+1, self.j0));
            },
            6 => {
                self.count = 7;
                return Some((self.i0+1, self.j0-1));
            },
            7 => {
                self.count = 8;
                return Some((self.i0, self.j0-1));
            }

            _ => return None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mask {
    size: usize,
    data: Vec<bool>,
}

impl Mask {
    pub fn new(size: usize) -> Mask {
        Mask {
            size,
            data: vec![true; size * size],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        self.data[self.size * i + j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: bool) {
        self.data[self.size * i + j] = value;
    }

    pub fn from_board(board: &Board) -> Mask {
        let mut mask = Mask::new(board.size());
        for j in 0..board.size() {
            for i in board.rows(j)..board.size() {
                mask.set(i, j, false);
            }
        }
        mask
    }

    pub fn neighbours(&self, i: usize, j: usize) -> Neighbours {
        Neighbours { count: 0, size: self.size, i0: i, j0: j }
    }

    pub fn neighbours2(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut x: Vec<(usize, usize)> = vec![];
        if i > 0 {
            if j > 0 && self.get(i - 1, j - 1) {
                x.push((i - 1, j - 1));
            }
            if self.get(i - 1, j) {
                x.push((i - 1, j));
            }
            if j + 1 < self.size && self.get(i - 1, j + 1) {
                x.push((i - 1, j + 1))
            }
        }
        if j > 0 && self.get(i, j - 1) {
            x.push((i, j - 1));
        }
        if j + 1 < self.size && self.get(i, j + 1) {
            x.push((i, j + 1))
        }
        if i + 1 < self.size {
            if j > 0 && self.get(i + 1, j - 1) {
                x.push((i + 1, j - 1));
            }
            if self.get(i + 1, j) {
                x.push((i + 1, j));
            }
            if j + 1 < self.size && self.get(i + 1, j + 1) {
                x.push((i + 1, j + 1))
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {
    // use boards::Board;
    use board::{Neighbours, Mask};

    #[test]
    fn neighbours() {

        let n = Neighbours { count: 0, size: 3, i0: 0, j0:0 };
        for (i, j) in n {
            println!("{}, {}", i, j);
        }
        // for println!("{}", n.collect());
    }
}
