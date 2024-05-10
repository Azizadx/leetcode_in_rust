impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut seen = HashSet::new();

        for num in nums {
            if seen.contains(&num) {
                return true;
            }
            seen.insert(num);
        }
        false
    }
}