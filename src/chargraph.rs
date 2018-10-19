use std::collections::HashMap;

pub struct CharGraph {
    pub isword : bool,
    pub subgraphs : HashMap<char, CharGraph>,
}


impl CharGraph {
    pub fn new() -> CharGraph {
        CharGraph {isword: false, subgraphs: HashMap::new()}
    }

    pub fn from_strings(words: &[&String]) -> CharGraph {
        let mut graph = CharGraph::new();
        for word in words {
            graph.push(word);
        }
        return graph;
    }

    pub fn push(&mut self, word: &str) {
        if word.len()==0 {
            self.isword = true;
        } else {
            let subgraph = self.subgraphs.entry(word.chars().next().unwrap())
                                        .or_insert(CharGraph::new());
            subgraph.push(&word[1..]);
        }
    }
}


#[cfg(test)]
mod tests {
    use chargraph::CharGraph;
    use dictionary::load;

    #[test]
    fn it_works() {
        let mut graph = CharGraph::new();
        graph.push("a");
        graph.push("aber");
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
