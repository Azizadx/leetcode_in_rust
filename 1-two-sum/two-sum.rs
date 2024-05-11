use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&comp_index) = num_map.get(&complement) {
                return vec![comp_index as i32, index as i32];
            }

            num_map.insert(num, index);
          }
        vec![]
    }
}