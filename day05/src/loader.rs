use std::fs;

use regex::Regex;

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
        .expect("Cannot read the input")
        .split('\n')
        .map(String::from)
        .collect();

    let end_of_stacks_definition = get_end_of_stacks_definition(&data_from_file);
    let stacks = parse_initial_stacks_state(&data_from_file, end_of_stacks_definition);
    let movements = parse_movements(&data_from_file, end_of_stacks_definition);

    Input { stacks , movements }
}

fn get_end_of_stacks_definition(data_from_file: &Vec<String>) -> u32 {
    for idx in 0..data_from_file.len() {
        match data_from_file[idx].chars().nth(1).unwrap().to_digit(10) {
            Some(_) => return idx as u32,
            None => continue,
        }
    }

    panic!("Apparently, the list of stacks has no end.");
}

fn parse_initial_stacks_state(data_from_file: &Vec<String>, end_of_stacks_definition: u32) -> Vec<CharStack> {
    let mut stacks: Vec<CharStack> = Vec::new();

    let num_of_stacks = data_from_file[end_of_stacks_definition as usize].chars()
        .nth_back(1)
        .unwrap()
        .to_digit(10)
        .unwrap();

    for _ in 1..(num_of_stacks + 1) {
        stacks.push(CharStack::new());
    }

    for idx in (0..(end_of_stacks_definition + 1)).rev() {
        let row = data_from_file[idx as usize].as_bytes();

        let mut current_stack_index: usize = 0;
        let mut possible_crate_index: usize = 1;
        while possible_crate_index < row.len() {
            let ch = row[possible_crate_index];
            if ch.is_ascii_alphabetic() && ch != ' ' as u8 {
                stacks[current_stack_index].push(ch as char);
            }
            possible_crate_index += 4;
            current_stack_index += 1;
        }
    }
    stacks
}

fn parse_movements(data_from_file: &Vec<String>, end_of_stacks_definition: u32) -> Vec<Movement> {
    let mut movements: Vec<Movement> = Vec::new();
    let start_of_movement = (end_of_stacks_definition + 2) as usize;

    let movement_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for row in &data_from_file[start_of_movement..] {
        if row.len() == 0 {
            continue;
        }

        let captures = match movement_regex.captures(row) {
            Some(cap) => cap,
            None => panic!("Cannot parse row {}", row),
        };

        movements.push(Movement {
            element_to_move: captures.get(1).map_or("", |m| m.as_str()).parse::<u32>().unwrap(),
            origin_stack: captures.get(2).map_or("", |m| m.as_str()).parse::<u32>().unwrap(),
            destination_stack: captures.get(3).map_or("", |m| m.as_str()).parse::<u32>().unwrap(),
        });
    }

    movements
}
