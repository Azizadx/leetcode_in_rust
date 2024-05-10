use std::collections::HashMap;
impl Solution {

    pub fn is_anagram(s: String, t: String) -> bool {
        // early exit
        if s.len() != t.len() {
            return false ;
        }
        let mut counts = HashMap::new();

        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            let count = counts.entry(ch).or_insert(0);
            *count -= 1;
            if *count < 0 {
                return false;
            }
        }

        true

    }
}