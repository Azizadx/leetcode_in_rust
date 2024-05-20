impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        //two pointer i to track the position of the next element to keep.
        // j to iterate of the loop 
        // both of i and j start from the same index which zero

        let mut i = 0;
        for j in 0..nums.len(){
            if nums[j] != val {
                //move the element 
                nums[i] = nums[j];

                i+=1;
            }
        }
        i as i32
    }
}