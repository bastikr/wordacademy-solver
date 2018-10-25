// fn reduced_wordlengths(wordlengths: &Vec<u8>, l: u8) -> Vec<u8> {
//     let removed = false;
//     let mut w : Vec<u8> = Vec::with_capacity(wordlengths.len()-1);
//     for x in wordlengths {
//         if *x==l && !removed {
//             continue;
//         }
//         w.push(*x);
//     }
//     w
// }

// fn arbitrary_solutions<'a>(wordlengths: Vec<u8>, words: &[&'a String], letters : HashMap<char, u8>) -> Vec<Vec<&'a String>> {
//     let reduced_words : Vec<&'a String> = words.into_iter().filter(|x| wordlengths.contains(&(x.len() as u8)))
//                             .filter(|x| writeable_with(x, &letters)).map(|x| *x)
//                             .collect();
//     // println!("{}: {}", wordlengths.len(), reduced_words.len());
//     let mut solutions = Vec::new();
//     if wordlengths.len()==1 {
//         for word in reduced_words {
//             solutions.push(vec![word]);
//         }
//         return solutions;
//     }
//     for (i, word) in reduced_words.iter().enumerate() {
//         let reduced_letters = reduced_histogram(&letters, &word);
//         let reduced_wordlengths = reduced_wordlengths(&wordlengths, word.len() as u8);
//         let subsolutions = arbitrary_solutions(reduced_wordlengths, &reduced_words[i..], reduced_letters);
//         for mut sol in subsolutions {
//             sol.push(word);
//             solutions.push(sol);
//         }
//     }
//     solutions
// }
