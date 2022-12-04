use std::{fs, collections::HashSet};

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Cannot read the input")
        .trim()
        .split('\n')
        .map(String::from)
        .collect()
}

fn find_overlap_item_in_backpack(first_half: &String, second_half: &String) -> char {
    let items_in_first_half: HashSet<char> = first_half.chars().collect();
    let items_in_second_half: HashSet<char> = second_half.chars().collect();

    let mut items_in_both: Vec<&char>  = items_in_first_half.intersection(&items_in_second_half).collect();

    if items_in_both.len() > 1 {
        panic!("I expect only an element in common between the two halfs");
    }

    items_in_both.pop().expect("No element in common between the two half").clone()
}

fn calculate_priority(overlap_item: char) -> u32 {
    let overlap_code = overlap_item as u32;
    let lower_a_code = 'a' as u32;
    let upper_a_code = 'A' as u32;

    if overlap_item.is_lowercase() {
        overlap_code - lower_a_code + 1
    } else {
        overlap_code - upper_a_code + 27
    }
}

fn step_1(backpacks: &Vec<String>) -> u32 {
    backpacks.iter()
        .map(|backpack| -> (String, String) {
            let half = backpack.len() / 2;
            let first_half = String::from(&backpack[..half]);
            let second_half = String::from(&backpack[half..]);

            (first_half, second_half)
        })
        .map(|backpack_content| find_overlap_item_in_backpack(&backpack_content.0, &backpack_content.1))
        .map(calculate_priority)
        .sum()
}

fn find_common_item_in_backpacks(backpacks: &[String]) -> char {
    let mut backpacks_items: Vec<HashSet<char>> = backpacks
        .to_vec()
        .iter()
        .map(|backpack| backpack.chars().collect::<HashSet<char>>())
        .collect();

    let (intersection, others) = backpacks_items.split_at_mut(1);
    let intersection = &mut intersection[0];

    for backpack in others {
       intersection.retain(|item| backpack.contains(item));
    }

    if intersection.len() > 1 {
        panic!("I expect only an element in common in a single group");
    }

    intersection.iter().next().unwrap().clone()
}

fn step_2(backpacks: &Vec<String>) -> u32 {
    backpacks.chunks(3)
        .map(find_common_item_in_backpacks)
        .map(calculate_priority)
        .sum()
}

fn main() {
    let input = read_input("input.txt");

    assert_eq!(step_1(&input), 7795);
    assert_eq!(step_2(&input), 2703);
}

