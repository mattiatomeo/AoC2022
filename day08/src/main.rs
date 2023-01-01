use std::fs;

#[derive(Debug)]
struct TreeGrid {
    height: Vec<Vec<u32>>,
    visible: Vec<Vec<u32>>,
}

const VISIBLE_TOP: u32 = 1 << 0;
const VISIBLE_BELOW: u32 = 1 << 1;
const VISIBLE_LEFT: u32 = 1 << 2;
const VISIBLE_RIGHT: u32 = 1 << 3;


impl TreeGrid {
    fn from_file(filename: &str) -> TreeGrid {
        let height: Vec<Vec<u32>> = fs::read_to_string(filename).expect("Cannot read the input")
            .split("\n")
            .filter(|row| row.len() > 0)
            .map(|row| row.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        let tree_in_row = height[0].len();
        let mut visible: Vec<Vec<u32>> = Vec::new();
        for _ in 0..height.len() {
            let mut row: Vec<u32> = Vec::new();
            row.resize(tree_in_row, 0);
            visible.push(row);
        }

        TreeGrid { height, visible }
    }

    fn detect_visible_trees(&mut self) {
        for row in 0..self.height.len() {
            for col in 0..self.height[0].len() {
                if self.is_visible_from_top(row, col) {
                    self.visible[row][col] |= VISIBLE_TOP;
                }

                if self.is_visible_from_below(row, col) {
                    self.visible[row][col] |= VISIBLE_BELOW;
                }

                if self.is_visible_from_left(row, col) {
                    self.visible[row][col] |= VISIBLE_LEFT;
                }

                if self.is_visible_from_right(row, col) {
                    self.visible[row][col] |= VISIBLE_RIGHT;
                }
            }
        }
    }

    fn is_visible_from_top(&self, row: usize, col: usize) -> bool {
        if row == 0 {
            return true;
        }

        let current_tree_height = self.height[row][col];
        for idx in (0..row).rev() {
            if self.height[idx][col] >= current_tree_height {
                return false;
            }
            if (self.visible[idx][col] & VISIBLE_TOP) > 0 {
                break;
            }
        }

        true
    }

    fn is_visible_from_below(&self, row: usize, col: usize) -> bool {
        let num_rows = self.height.len();
        if row == num_rows - 1 {
            return true;
        }

        let current_tree_height = self.height[row][col];
        for idx in (row+1)..num_rows {
            if self.height[idx][col] >= current_tree_height {
                return false;
            }
            if (self.visible[idx][col] & VISIBLE_BELOW) > 0 {
                break;
            }
        }

        true
    }

    fn is_visible_from_left(&self, row: usize, col: usize) -> bool {
        if col == 0 {
            return true;
        }

        let current_tree_height = self.height[row][col];
        for idx in (0..col).rev() {
            if self.height[row][idx] >= current_tree_height {
                return false;
            }
            if (self.visible[row][idx] & VISIBLE_LEFT) > 0 {
                break;
            }
        }

        true
    }

    fn is_visible_from_right(&self, row: usize, col: usize) -> bool {
        let num_cols = self.height[0].len();
        if col == num_cols - 1 {
            return true;
        }

        let current_tree_height = self.height[row][col];
        for idx in (col+1)..num_cols {
            if self.height[row][idx] >= current_tree_height {
                return false;
            }
            if (self.visible[row][idx] & VISIBLE_RIGHT) > 0 {
                break;
            }
        }

        true
    }

    fn get_total_visible_trees(&self) -> u32 {
        self.visible.iter()
            .map(|row| row.iter().filter(|&visible| *visible > 0).count() as u32)
            .sum::<u32>()
    }
}

fn step_1(filename: &str) -> u32 {
    let mut trees = TreeGrid::from_file(filename);
    trees.detect_visible_trees();

    //println!("{:#?}", trees);
    trees.get_total_visible_trees()
}

fn main() {
    assert_eq!(step_1("input.txt"), 1779);
}
