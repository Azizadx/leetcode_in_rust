use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        //Early exit 
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut mag_count = HashMap::new();

        for char in magazine.chars() {
            *mag_count.entry(char).or_insert(0) += 1;
        }

        for char in ransom_note.chars() {
            let count = mag_count.entry(char).or_default();
            if *count == 0 {
                return false;
            }
            *count  -= 1;
        }
        true
    }
}