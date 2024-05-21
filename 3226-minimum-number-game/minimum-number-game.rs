impl Solution {
pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
    // Duplicate the nums vector
    let mut nums = nums.clone();
    nums.sort(); // Sort the numbers
    let mut arr = Vec::new();

    while !nums.is_empty() {
        // Alice's turn to pick and append
        let alice_pick = nums.remove(0);
        if let Some(bob_pick) = nums.get(0) {
            // Bob's turn to pick
            let bob_pick = *bob_pick;
            nums.remove(0);

            // Bob appends first, then Alice
            arr.push(bob_pick);
        }
        arr.push(alice_pick);
    }

    arr
    }
}