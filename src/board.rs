use std::fmt;

#[derive(Clone, Debug)]
pub struct Word {
    pub chars : Vec<char>,
    pub coordinates : Vec<(usize, usize)>,
}

impl Word {
    pub fn new() -> Word {
        return  Word {chars: vec![], coordinates: vec![]};
    }

    pub fn add(&self, i: usize, j: usize, x: char) -> Word {
        let mut w = self.clone();
        w.chars.push(x);
        w.coordinates.push((i, j));
        return w;
    }

    pub fn as_string(&self) -> String {
        let mut s = String::with_capacity(self.chars.len());
        for x in self.chars.iter() {
            s.push(*x);
        }
        s
    }

    fn contains_coordinates(&self, i: usize, j: usize) -> bool {
        for (i_word, j_word) in self.coordinates.iter() {
            if *i_word==i && *j_word==j {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct Board {
    pub data: Vec<Vec<char>>,
}

impl Board {
    pub fn from_string(boardstring: &str) -> Board {
        let size = (boardstring.len() as f64).sqrt() as usize;
        assert!(size*size == boardstring.len());
        let chars : Vec<char> = boardstring.chars().collect();
        let mut data : Vec<Vec<char>> = vec![];
        for j in 0..size {
            let mut column : Vec<char> = Vec::with_capacity(size);
            for i in (0..size).rev() {
                column.push(chars[size*i + j]);
            }
            data.push(column);
        }
        Board { data: data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn rows(&self, j: usize) -> usize {
        self.data[j].len()
    }

    pub fn get(&self, i: usize, j: usize) -> char {
        self.data[j][i]
    }

    pub fn reduce(&self, word: &Word) -> Board {
        let size = self.data.len();
        let mut data : Vec<Vec<char>> = vec![];
        for j in 0..size {
            let mut column : Vec<char> = Vec::with_capacity(self.data[j].len());
            for i in 0..self.data[j].len() {
                if word.contains_coordinates(i, j) {
                    continue;
                }
                column.push(self.get(i, j));
            }
            data.push(column);
        }
        Board { data: data }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut separator = "+-".repeat(self.size());
        separator.push_str("+");
        writeln!(f, "");
        for i in (0..self.size()).rev() {
            writeln!(f, "{}", separator);
            for j in 0..self.size() {
                write!(f, "|");
                if self.rows(j)>i {
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

#[derive(Clone, Debug)]
pub struct Mask {
    size : usize,
    data : Vec<bool>,
}

impl Mask {
    pub fn new(size: usize) -> Mask {
        Mask {size: size, data: vec![true; size*size ]}
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        self.data[self.size*j + i]
    }

    pub fn set(&mut self, i: usize, j: usize, value: bool) {
        self.data[self.size*j + i] = value;
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

    pub fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut x: Vec<(usize, usize)> = vec![];
        if i>0 {
            if j>0 && self.get(i-1, j-1) {
                x.push((i-1, j-1));
            }
            if self.get(i-1, j) {
                x.push((i-1, j));
            }
            if j+1<self.size && self.get(i-1, j+1) {
                x.push((i-1, j+1))
            }
        }
        if j>0 && self.get(i, j-1) {
            x.push((i, j-1));
        }
        if j+1<self.size && self.get(i, j+1) {
            x.push((i, j+1))
        }
        if i+1<self.size {
            if j>0 && self.get(i+1, j-1) {
                x.push((i+1, j-1));
            }
            if self.get(i+1, j) {
                x.push((i+1, j));
            }
            if j+1<self.size && self.get(i+1, j+1) {
                x.push((i+1, j+1))
            }
        }
        x
    }
}


#[cfg(test)]
mod tests {
    // use boards::Board;
    // use boards::Mask;

    // #[test]
    // fn mask() {
        
    // }
}
