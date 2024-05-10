impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for char in s.chars() {
            match char {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if stack.pop() != Some(char){
                        return false;
                    }
                },
                _ => return false,
            }
        }
        stack.is_empty()
    }
}