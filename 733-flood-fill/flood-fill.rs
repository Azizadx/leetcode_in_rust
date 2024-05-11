/*
4-Directional Connectivity Equation for 2D
    North (Up): (i-1, j)
    South (Down): (i+1, j)
    East (Right): (i, j+1)
    West (Left): (i, j-1)

*/

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let current_color = image[sr as usize][sc as usize]; //get the current color for the starting point
        if current_color != color {
            Solution::fill(&mut image, sr,sc, color, current_color);
        }
        image
    }

    fn fill(image: &mut Vec<Vec<i32>>, sr: i32, sc:i32, new_color: i32, old_color: i32) {
        // check if starting row Or Starting column are not out of bound or we are outside the image frame for that pixels
        if sr < 0 || sc < 0 || sr >= image.len() as i32 || sc >= image[0].len() as i32 || image[sr as usize][sc as usize] != old_color {
            return;
        }
        //change the color 
        image[sr as usize][sc as usize] = new_color;

        Self::fill(image, sr + 1, sc, new_color, old_color); // south
        Self::fill(image, sr - 1, sc, new_color, old_color); // north
        Self::fill(image, sr, sc + 1, new_color, old_color); // east
        Self::fill(image, sr, sc - 1, new_color, old_color); // west
  }
    
}