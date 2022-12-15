use std::fs;

use crate::stack::CharStack;

#[derive(Debug)]
pub struct Movement {
    pub element_to_move: u32,
    pub origin_stack: u32,
    pub destination_stack: u32,
}

#[derive(Debug)]
pub struct Input {
    pub stacks: Vec<CharStack>,
    pub movements: Vec<Movement>,
}


pub fn read_input(filename: &str) -> Input {
    let data_from_file = fs::read_to_string(filename)
        .expect("Cannot read the input");

    let end_of_stack_definition = get_end_of_stacks_definition(&data_from_file);
    let stacks = parse_initial_stacks_state(&data_from_file, end_of_stack_definition);
    let movements = parse_movements(&data_from_file, end_of_stack_definition);

    Input { stacks , movements }
}

fn get_end_of_stacks_definition(data_from_file: &String) -> u32 {
    0
}

fn parse_initial_stacks_state(data_from_file: &String, end_of_stack_definition: u32) -> Vec<CharStack> {
    let mut stacks: Vec<CharStack> = Vec::new();
    stacks.push(CharStack::new());

    stacks
}

fn parse_movements(data_from_file: &String, end_of_stack_definition: u32) -> Vec<Movement> {
    let mut movements: Vec<Movement> = Vec::new();
    movements.push(Movement { element_to_move: 0, origin_stack: 0, destination_stack: 0 });

    movements
}
