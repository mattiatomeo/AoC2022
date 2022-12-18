mod loader;
mod stack;

use crate::loader::Input;
use loader::Movement;
use stack::CharStack;


fn pop_elements_from_stack(stacks: &mut Vec<CharStack>, movement: &Movement) -> Vec<char> {
    let mut crates_in_origin: Vec<char> = Vec::new();
    let origin = &mut stacks[(movement.origin_stack - 1) as usize];

    for _ in 0..movement.element_to_move {
        crates_in_origin.push(origin.pop());
    }
    crates_in_origin
}

fn push_elements_in_stack(stacks: &mut Vec<CharStack>, movement: &Movement, crates: Vec<char>) {
    let destination = &mut stacks[(movement.destination_stack - 1) as usize];
    for ch in crates {
        destination.push(ch);
    }
}

fn execute_movement(stacks: &mut Vec<CharStack>, movement: &Movement) {
    let crates_to_move = pop_elements_from_stack(stacks, movement);
    push_elements_in_stack(stacks, movement, crates_to_move);
}

fn apply_movements(input: &mut Input) {
    for movement in &input.movements {
        execute_movement(&mut input.stacks, movement);
    }
}

fn step_1() -> String {
    let mut input = loader::read_input("input.txt");
    apply_movements(&mut input);

    let mut crates_on_top = String::new();
    for crate_on_top in input.stacks.iter().map(CharStack::top) {
        crates_on_top.push(crate_on_top);
    }

    crates_on_top
}

fn main() {
    assert_eq!(step_1(), "ZRLJGSCTR");
}
