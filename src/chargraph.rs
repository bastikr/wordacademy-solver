use std::collections::HashMap;


#[derive(Debug, Default)]
pub struct CharGraph {
    isword: bool,
    lengths: [bool; 15],
    pub chars: Vec<char>,
    pub subgraphs: Vec<CharGraph>,

    // pub subgraphs: HashMap<char, CharGraph>,
}

impl CharGraph {
    pub fn new() -> CharGraph {
        CharGraph {
            isword: false,
            lengths: [false; 15],
            chars: vec![],
            subgraphs: vec![],
        }
    }

    pub fn isword(&self) -> bool {
        self.isword
    }

    pub fn from_strings(words: &[String]) -> CharGraph {
        let mut graph = CharGraph::new();
        for word in words {
            graph.push(word, word.len());
        }
        graph
    }

    pub fn push(&mut self, word: &str, length: usize) {
        if word.is_empty() {
            self.isword = true;
        } else {
            let x = word.chars().next().unwrap();
            let subgraph = match self.chars.binary_search(&x) {
                Ok(index) => &mut self.subgraphs[index],
                Err(index) => { self.chars.insert(index, x); self.subgraphs.insert(index, CharGraph::new()); &mut self.subgraphs[index] }
            };
            if length < 15 {
                self.lengths[length] = true;
            }
            subgraph.push(&word[1..], length);
        }
    }

    pub fn contains_length(&self, length: usize) -> bool {
        if length > 15 {
            return true;
        }
        self.lengths[length]
    }

    pub fn contains_key(&self, x: &char) -> bool {
        match self.chars.binary_search(x) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn sort(&mut self) {

    }

    pub fn subgraph(&self, x: &char) -> &CharGraph {
        let subgraph = match self.chars.binary_search(&x) {
            Ok(index) => &self.subgraphs[index],
            Err(index) => { panic!("bla") }
        };
        &subgraph
    }
}

#[cfg(test)]
mod tests {
    use chargraph::CharGraph;
    use dictionary::load;

    #[test]
    fn it_works() {
        let mut graph = CharGraph::new();
        graph.push("a", 1);
        graph.push("aber", 4);
        assert_eq!(graph.subgraphs.len(), 1);
        // assert_eq!(graph.subgraphs, 1);
    }

    #[test]
    fn build_chargraph_from_dict() {
        let words_lower = load("/usr/share/dict/cracklib-small");
        let words = words_lower.into_iter().collect::<Vec<String>>();
        let graph = CharGraph::from_strings(&words);
    }
}
