mod loader;
mod stack;

fn main() {
    println!("{:?}", loader::read_input("input.txt"));
}
