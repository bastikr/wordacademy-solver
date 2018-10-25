use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CharGraph {
    pub isword : bool,
    lengths : [bool; 15],
    pub subgraphs : HashMap<char, CharGraph>,
}


impl CharGraph {
    pub fn new() -> CharGraph {
        CharGraph {isword: false, lengths : [false; 15], subgraphs: HashMap::new()}
    }

    pub fn from_strings(words: &[&String]) -> CharGraph {
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
            let subgraph = self.subgraphs.entry(word.chars().next().unwrap())
                                         .or_insert(CharGraph::new());
            if length<15 {
                self.lengths[length] = true;
            }
            subgraph.push(&word[1..], length);
        }
    }

    pub fn contains_length(&self, length: usize) -> bool {
        if length>15 {
            return true;
        }
        self.lengths[length]
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
        let words = words_lower.iter().collect::<Vec<&String>>();
        let graph = CharGraph::from_strings(&words);
    }
}
