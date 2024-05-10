impl Solution {
    #[inline]
    pub fn is_palindrome(s: String) -> bool {
        //convert the string to char
        let chars: Vec<char> = s.chars().collect();
        let mut left_pointer = 0;
        let mut right_pointer = chars.len() - 1;

        while left_pointer < right_pointer {
            // Skip non-alphanumeric from left pointer
            if !Solution::is_alphanumeric(chars[left_pointer]) {
                left_pointer +=1;
                continue;
            }
            // Skip non-alphanumeric from right pointer
            if !Solution::is_alphanumeric(chars[right_pointer]) {
                right_pointer -=1;
                continue;
            }

            if Solution::is_lowercase(chars[left_pointer]) != Solution::is_lowercase(chars[right_pointer]){
                return false;
            }

            left_pointer += 1;
            right_pointer -=1;
        }
        true
    }
    // helper functions
    #[inline]
    fn is_alphanumeric(c: char) -> bool {
        (c >= 'a' && c<='z') ||  (c >= 'A' && c<='Z') ||  (c >= '0' && c<='9')
    }
    #[inline]
    fn is_lowercase(c: char) -> char {
        if c >= 'A' && c <= 'Z' {
            return (c as u8 + 32) as char;
        }
        c
    }

    
}