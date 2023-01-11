use std::{fs, collections::HashSet, hash::Hash};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom
}

#[derive(Debug)]
struct Movement {
    pub direction: Direction,
    pub step: i32,
}

fn read_input(filename: &str) -> Vec<Movement> {
    fs::read_to_string(filename).expect("Cannot read the input")
        .split("\n")
        .filter(|row| row.len() > 0)
        .map(|row| -> Movement {
            let mut splitted = row.split(" ");
            let direction = match splitted.next().unwrap() {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Top,
                "D" => Direction::Bottom,
                _ => panic!("Unknown direction"),
            };

            let step = splitted.next().unwrap().parse::<i32>().unwrap();

            Movement { direction , step }
        }).collect()
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}


struct TwoKnotsRope {
    head: Point,
    tail: Point,

    trail_history: HashSet<Point>
}

impl TwoKnotsRope {
    fn new() -> TwoKnotsRope {
        TwoKnotsRope { head: Point { x: 0, y: 0 }, tail: Point { x: 0, y: 0 }, trail_history: HashSet::new() }
    }

    fn move_head(&mut self, head_movement: &Vec<Movement>) {
        for movement in head_movement {
            match movement.direction {
                Direction::Left => {
                    for _ in 0..movement.step {
                        self.move_head_to_left();
                        self.trail_history.insert(self.tail.clone());
                    }
                }
                Direction::Right => {
                    for _ in 0..movement.step {
                        self.move_head_to_right();
                        self.trail_history.insert(self.tail.clone());
                    }
                }
                Direction::Top => {
                    for _ in 0..movement.step {
                        self.move_head_to_top();
                        self.trail_history.insert(self.tail.clone());
                    }
                }
                Direction::Bottom => {
                    for _ in 0..movement.step {
                        self.move_head_to_bottom();
                        self.trail_history.insert(self.tail.clone());
                    }
                }
            }
        }
    }

    fn move_head_to_right(&mut self) {
        self.head.y += 1;

        if self.are_head_and_tail_adjacent() {
            return
        }

        self.tail.y = self.head.y - 1;

        if self.are_head_and_tail_in_different_row_and_col() {
            self.tail.x = self.head.x;
        }
    }

    fn move_head_to_left(&mut self) {
        self.head.y -= 1;

        if self.are_head_and_tail_adjacent() {
            return
        }

        self.tail.y = self.head.y + 1;

        if self.are_head_and_tail_in_different_row_and_col() {
            self.tail.x = self.head.x;
        }
    }

    fn move_head_to_top(&mut self) {
        self.head.x += 1;

        if self.are_head_and_tail_adjacent() {
            return
        }

        self.tail.x = self.head.x - 1;

        if self.are_head_and_tail_in_different_row_and_col() {
            self.tail.y = self.head.y;
        }
    }

    fn move_head_to_bottom(&mut self) {
        self.head.x -= 1;

        if self.are_head_and_tail_adjacent() {
            return
        }

        self.tail.x = self.head.x + 1;

        if self.are_head_and_tail_in_different_row_and_col() {
            self.tail.y = self.head.y;
        }
    }

    fn are_head_and_tail_adjacent(&self) -> bool {
        let x_dist = self.head.x.abs_diff(self.tail.x);
        let y_dist = self.head.y.abs_diff(self.tail.y);

        x_dist <= 1 && y_dist <= 1
    }

    fn are_head_and_tail_in_different_row_and_col(&self) -> bool {
        self.head.x != self.tail.x && self.head.y != self.tail.y
    }

    fn get_position_traversed_by_tail(&self) -> HashSet<Point> {
        self.trail_history.clone()
    }
}

struct TenKnotsRope {
    knots: Vec<Point>,
    trail_history: HashSet<Point>
}

impl TenKnotsRope {
    fn new() -> TenKnotsRope {
        TenKnotsRope { knots: (0..10).map(|_| Point { x: 0, y: 0 }).collect(), trail_history: HashSet::new() }
    }

    fn move_head(&mut self, head_movement: &Vec<Movement>) {
        for movement in head_movement {
            match movement.direction {
                Direction::Left => {
                    for _ in 0..movement.step {
                        self.move_head_to_left();
                        self.trail_history.insert(self.knots.iter().last().unwrap().clone());
                    }
                }
                Direction::Right => {
                    for _ in 0..movement.step {
                        self.move_head_to_right();
                        self.trail_history.insert(self.knots.iter().last().unwrap().clone());
                    }
                }
                Direction::Top => {
                    for _ in 0..movement.step {
                        self.move_head_to_top();
                        self.trail_history.insert(self.knots.iter().last().unwrap().clone());
                    }
                }
                Direction::Bottom => {
                    for _ in 0..movement.step {
                        self.move_head_to_bottom();
                        self.trail_history.insert(self.knots.iter().last().unwrap().clone());
                    }
                }
            }
        }
    }

    fn move_head_to_right(&mut self) {
        self.knots[0].y += 1;

        for idx in 1..self.knots.len() {
            let adjusted = TenKnotsRope::adjust_tail_if_not_adjacent(self.knots[idx - 1], &mut self.knots[idx]);
            if !adjusted {
                break;
            }
        }
    }

    fn adjust_tail_if_not_adjacent(head: Point, tail: &mut Point) -> bool {
        if TenKnotsRope::are_head_and_tail_adjacent(&head, tail) {
            return false;
        }

        let different_row = head.x != tail.x;
        let different_col = head.y != tail.y;

        let x_direction;
        if head.x > tail.x {
            x_direction = 1;
        } else {
            x_direction = -1;
        }

        let y_direction;
        if head.y > tail.y {
            y_direction = 1;
        } else {
            y_direction = -1;
        }

        if different_row && different_col {
            tail.x += 1 * x_direction;
            tail.y += 1 * y_direction as i32;
            return true;
        } else if different_row {
            tail.x += 1 * x_direction as i32;
            return true;
        } else if different_col {
            tail.y += 1 * y_direction as i32;
            return true;
        }

        false
    }

    fn move_head_to_left(&mut self) {
        self.knots[0].y -= 1;

        for idx in 1..self.knots.len() {
            TenKnotsRope::adjust_tail_if_not_adjacent(self.knots[idx - 1], &mut self.knots[idx]);
        }
    }

    fn move_head_to_top(&mut self) {
        self.knots[0].x += 1;

        for idx in 1..self.knots.len() {
            TenKnotsRope::adjust_tail_if_not_adjacent(self.knots[idx - 1], &mut self.knots[idx]);
        }
    }

    fn move_head_to_bottom(&mut self) {
        self.knots[0].x -= 1;

        for idx in 1..self.knots.len() {
            TenKnotsRope::adjust_tail_if_not_adjacent(self.knots[idx - 1], &mut self.knots[idx]);
        }
    }

    fn are_head_and_tail_adjacent(head: &Point, tail: &Point) -> bool {
        let x_dist = head.x.abs_diff(tail.x);
        let y_dist = head.y.abs_diff(tail.y);

        x_dist <= 1 && y_dist <= 1
    }

    fn get_position_traversed_by_tail(&self) -> HashSet<Point> {
        self.trail_history.clone()
    }
}

fn step_1(input: &Vec<Movement>) -> u32 {
    let mut rope = TwoKnotsRope::new();
    rope.move_head(input);

    rope.get_position_traversed_by_tail().len() as u32
}

fn step_2(input: &Vec<Movement>) -> u32 {
    let mut rope = TenKnotsRope::new();
    rope.move_head(input);

    rope.get_position_traversed_by_tail().len() as u32
}

fn main() {
    let input = read_input("input.txt");
    assert_eq!(6391, step_1(&input));
    assert_eq!(2593, step_2(&input));
}
