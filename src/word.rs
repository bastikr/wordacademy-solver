#[derive(Debug, Clone, Default)]
pub struct Word {
    pub chars: Vec<char>,
    pub coordinates: Vec<(usize, usize)>,
}

impl Word {
    pub fn new() -> Word {
        Word {
            chars: vec![],
            coordinates: vec![],
        }
    }

    pub fn add(&self, i: usize, j: usize, x: char) -> Word {
        let mut chars : Vec<char> = Vec::with_capacity(self.chars.len() + 1);
        let mut coordinates : Vec<(usize, usize)> = Vec::with_capacity(self.coordinates.len() + 1);
        chars.extend(&self.chars);
        chars.push(x);
        coordinates.extend(&self.coordinates);
        coordinates.push((i, j));
        Word { chars, coordinates }
    }

    pub fn as_string(&self) -> String {
        let mut s = String::with_capacity(self.chars.len());
        for x in &self.chars {
            s.push(*x);
        }
        s
    }

    pub fn contains_coordinates(&self, i: usize, j: usize) -> bool {
        for (i_word, j_word) in &self.coordinates {
            if *i_word == i && *j_word == j {
                return true;
            }
        }
        false
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    // pub fn previous_coordinates(&self, previous_word: &Word) -> bool {
    //     let mut c = self.coordinates.clone();
    //     for coordinates in previous_word.coordinates
    // }

    pub fn droplength(&self, i0: usize, j0: usize) -> usize {
        let mut d: usize = 0;
        for (i, j) in self.coordinates.iter() {
            if (*i)==i0 && (*j)<j0 {
                d += 1;
            }
        }
        d
    }

    pub fn commutates_with_previous(&self, previous_word: &Word) -> bool {
        let mut j_last: Option<usize> = None;
        for (i, j) in self.coordinates.iter() {
            let j_now = j - previous_word.droplength(*i, *j);
            match j_last {
                Some(value) => {
                    if value+1<j_now || j_now+1<value {
                        return false;
                    }
                }
                None => {}
            }
            j_last = Some(j_now);
        }
        true
    }

    pub fn is_before_previous(&self, previous_word: &Word) -> bool {
        if self.coordinates[0].1<previous_word.coordinates[0].1 {
            return true;
        }
        if self.coordinates[0].1>previous_word.coordinates[0].1 {
            return false;
        }
        self.coordinates[0].0 + previous_word.droplength(self.coordinates[0].0, self.coordinates[0].1) < previous_word.coordinates[0].0
    }
}
