impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
       helper(n, true)
    }
}

 fn helper (n: i32, left:bool) -> i32 {
        //early exit if n ==0 
        if n == 0 {
            return 0;
        }
        //early exit if n is only one index
        if n == 1 {
            return 1;
        }
        if left || n %2 == 1 {
            return 2 * helper(n/2, !left);
        }
        else {
            return 2 * helper( n/2, !left) - 1;
        }
    } 