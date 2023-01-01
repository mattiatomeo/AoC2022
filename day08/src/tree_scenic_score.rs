use std::fs;

#[derive(Debug)]
struct TreeGrid {
    height: Vec<Vec<u32>>,
    scenic_score: Vec<Vec<u32>>,
}

impl TreeGrid {
    fn from_file(filename: &str) -> TreeGrid {
        let height: Vec<Vec<u32>> = fs::read_to_string(filename).expect("Cannot read the input")
            .split("\n")
            .filter(|row| row.len() > 0)
            .map(|row| row.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        let tree_in_row = height[0].len();
        let mut scenic_score: Vec<Vec<u32>> = Vec::new();
        for _ in 0..height.len() {
            let mut row: Vec<u32> = Vec::new();
            row.resize(tree_in_row, 0);
            scenic_score.push(row);
        }

        TreeGrid { height, scenic_score }
    }

    fn calculate_scenic_score(&mut self) {
        for row in 1..self.height.len() {
            for col in 1..self.height[0].len() {
                let mut scenic_score = 1;

                scenic_score *= self.get_visible_tree_at_top(row, col);
                scenic_score *= self.is_visible_from_below(row, col);
                scenic_score *= self.is_visible_from_left(row, col);
                scenic_score *= self.is_visible_from_right(row, col);

                self.scenic_score[row][col] = scenic_score;
            }
        }
    }

    fn get_visible_tree_at_top(&self, row: usize, col: usize) -> u32 {
        let current_tree_height = self.height[row][col];
        let mut visible_from_top = 0;

        for idx in (0..row).rev() {
            visible_from_top += 1;
            if self.height[idx][col] >= current_tree_height {
                break;
            }
        }

        visible_from_top
    }

    fn is_visible_from_below(&self, row: usize, col: usize) -> u32 {
        let num_rows = self.height.len();
        let mut visible_from_below = 0;

        let current_tree_height = self.height[row][col];
        for idx in (row+1)..num_rows {
            visible_from_below += 1;
            if self.height[idx][col] >= current_tree_height {
                break;
            }
        }

        visible_from_below
    }

    fn is_visible_from_left(&self, row: usize, col: usize) -> u32 {
        let mut visible_from_left = 0;

        let current_tree_height = self.height[row][col];
        for idx in (0..col).rev() {
            visible_from_left += 1;
            if self.height[row][idx] >= current_tree_height {
                break;
            }
        }

        visible_from_left
    }

    fn is_visible_from_right(&self, row: usize, col: usize) -> u32 {
        let num_cols = self.height[0].len();
        let mut visible_from_right = 0;

        let current_tree_height = self.height[row][col];
        for idx in (col+1)..num_cols {
            visible_from_right += 1;
            if self.height[row][idx] >= current_tree_height {
                break;
            }
        }

        visible_from_right
    }

    fn get_max_scenic_score(&self) -> u32 {
        self.scenic_score.iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
            .to_owned()
    }
}


pub fn step_2(filename: &str) -> u32 {
    let mut trees = TreeGrid::from_file(filename);
    trees.calculate_scenic_score();

    trees.get_max_scenic_score()
}
