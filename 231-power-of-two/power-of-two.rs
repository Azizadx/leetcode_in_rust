/*
AND bitwise operation
e.g: n = 4
4 in binary = 0100
n & (n - 1)
n -1 = 3 = 0011
0100 & 0011 = 0000 which is true
e.g: 3 = 0011
n - 1 = 2 = 0010
0011 & 0010 = 0010 there 1 which is false
*/
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}