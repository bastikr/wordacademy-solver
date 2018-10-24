use std::collections::HashMap;
use board::Board;

pub struct CharHistogram {
    data: HashMap<char, u8>
}

impl CharHistogram {
    pub fn from_word(word: &str) -> CharHistogram {
        let mut h = HashMap::new();
        for letter in word.chars() {
            let count = h.entry(letter).or_insert(0);
            *count += 1;
        }
        CharHistogram {data: h}
    }

    pub fn from_board(board: &Board) -> CharHistogram {
        let mut h = HashMap::new();
        for j in 0..board.size() {
            for i in 0..board.rows(j) {
                let letter = board.get(i, j);
                let count = h.entry(letter).or_insert(0);
                *count += 1;
            }
        }
        CharHistogram {data: h}
    }

    pub fn contains(&self, subhistogram: &CharHistogram) -> bool {
        for (letter, subcount) in subhistogram.data.iter() {
            if match self.data.get(&letter) {
                Some(count_available) => subcount>count_available,
                None => true
            } {return false;}
        }
        return true;
    }

    pub fn writeable(&self, word: &str) -> bool {
        let wordhistogram = CharHistogram::from_word(word);
        self.contains(&wordhistogram)
    }

    pub fn substract(&self, word: &str) -> CharHistogram {
        let letter_hist = CharHistogram::from_word(word);
        let mut h : HashMap<char, u8> = HashMap::new();
        for (letter, count) in self.data.iter() {
            let c = match letter_hist.data.get(letter) {
                Some(letters_count) => *letters_count,
                None => 0
            };
            if count - c > 0 {
                h.insert(*letter, count-c);
            }
        }
        CharHistogram {data: h}
    }
}



