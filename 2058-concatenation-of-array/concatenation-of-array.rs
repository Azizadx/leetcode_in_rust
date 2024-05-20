impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        //setup n 
        let n = nums.len();
       let mut ans = vec![0; 2 * n]; // Initialize a vector of length 2n with zeros
       for i in 0..n {
        ans[i] = nums[i]; //first half
        ans[i+n] = ans[i]; //2nd half
       }
       ans

    }
}