use std::collections::HashSet;
impl Solution {
    #[inline]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();
        for num in nums {
            if !map.insert(num) {
                return true;
            }
        }
        false
    }
}