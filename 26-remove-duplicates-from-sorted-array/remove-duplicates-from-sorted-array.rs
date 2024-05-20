// Note: it already sorted
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
         //check it is empty early exit  
        if nums.is_empty(){return 0 ;}
        // two pointer soluation (slow_pointer, fast_pointer
        //slow_pointer only jump when the unique element is found
        let mut slow_pointer = 0;
        //faster pointer is always on the move 
        for faster_pointer in 1..nums.len(){
            if nums[faster_pointer] != nums[slow_pointer] {
                slow_pointer += 1;
                nums[slow_pointer] = nums[faster_pointer];
            }
        }

        (slow_pointer + 1) as i32
    }
}