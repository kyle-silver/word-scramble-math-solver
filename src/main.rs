use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
struct Permutation([u8; 26]);

impl From<&[char]> for Permutation {
    fn from(permutation: &[char]) -> Self {
        let mut perm = [0; 26];
        for (i, &character) in permutation.iter().enumerate() {
            let char_index = character as usize - 'a' as usize;
            perm[char_index] = i as u8;
        }
        Self(perm)
    }
}

impl Permutation {
    fn get(&self, c: char) -> u8 {
        self.0[c as usize - 'a' as usize]
    }

    fn word_sum(&self, word: &str) -> u32 {
        word.chars()
            .rev()
            .enumerate()
            .map(|(i, c)| self.get(c) as u32 * 10u32.pow(i as u32))
            .sum()
    }

    fn phrase_sum(&self, words: &[&str]) -> u32 {
        words.iter().map(|&word| self.word_sum(word)).sum()
    }

    fn check(&self) -> bool {
        let lhs = self.phrase_sum(&["we", "want", "no", "new", "atomic"]);
        let rhs = self.word_sum("weapon");
        lhs == rhs
    }
}

fn main() {
    let letters: HashSet<_> = "we want no new atomic weapon"
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    for (i, permutation_elements) in letters.into_iter().permutations(10).enumerate() {
        let permutation = Permutation::from(&permutation_elements[..]);
        if i % 100_000 == 0 {
            println!("At Guess #{i}");
        }
        if permutation.check() {
            println!("Found! (Guess #{i}) {permutation:?}");
            break;
        }
    }
}
