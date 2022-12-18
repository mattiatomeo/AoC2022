use std::{fs, collections::HashSet};

fn read_input(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(input) => input,
        Err(err) => panic!("Cannot read file {}: {}", filename, err),
    }
}

fn are_all_unique(window: &str) -> bool {
    let mut uniques: HashSet<char> = HashSet::new();

    uniques.extend(window.chars());

    uniques.len() == window.len()
}

fn step_1(input: &String) -> u32 {
    let unique_character_size_before_marker = 4;
    match find_marker(input, unique_character_size_before_marker) {
        Some(marker) => marker,
        None => panic!("Cannot find a marker give a unique character sequence size of {}", unique_character_size_before_marker),
    }
}

fn step_2(input: &String) -> u32 {
    let unique_character_size_before_marker = 14;
    match find_marker(input, unique_character_size_before_marker) {
        Some(marker) => marker,
        None => panic!("Cannot find a marker give a unique character sequence size of {}", unique_character_size_before_marker),
    }
}


fn find_marker(input: &String, unique_character_size_before_marker: u32) -> Option<u32> {
    let windows_size = unique_character_size_before_marker as usize;
    let mut end_of_window: usize = windows_size - 1;
    while end_of_window < input.len() {
        let window_start = end_of_window - (windows_size - 1);
        let next_after_window = end_of_window + 1;
        if are_all_unique(&input[window_start..next_after_window]) {
            return Some(next_after_window as u32)
        }
        end_of_window += 1;
    }

    None
}

fn main() {
    let input = read_input("input.txt");
    assert_eq!(1566, step_1(&input));
    assert_eq!(2265, step_2(&input));
}
