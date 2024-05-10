use std::collections::HashSet;
impl Solution {
    #[inline]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::<i32>::new();
        for num in nums {
            if seen.contains(&num) {
                return true;
            }
            seen.insert(num);
        }
        false
    }
}