mod tree_visibility;
mod tree_scenic_score;

fn main() {
    assert_eq!(tree_visibility::step_1("input.txt"), 1779);
    assert_eq!(tree_scenic_score::step_2("input.txt"), 172224);
}
