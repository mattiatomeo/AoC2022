use std::fs;


fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Cannot read the input")
}

fn split_meals_by_elves(elves_meals: &String) -> Vec<String> {
    elves_meals.split("\n\n").map(|meals| String::from(meals)).collect()
}   

fn calculate_total_calories_per_elves(elves_meals: &Vec<String>) -> Vec<i32> {
    let mut calories_per_elves = Vec::new();

    for chunck_meals in elves_meals {
        let calories: i32 = chunck_meals.split("\n").map(|meal| meal.parse::<i32>().unwrap_or_default()).sum();
        calories_per_elves.push(calories)
    }

    calories_per_elves
}

fn max_calories(calories_per_elves: &Vec<i32>) -> i32 {
    *calories_per_elves.iter().max().expect("Something goes wrong")
}

fn calculate_calories_per_elves(elves_calories_records: &String) -> Vec<i32> {
    let elves_meals = split_meals_by_elves(&elves_calories_records);

    calculate_total_calories_per_elves(&elves_meals)
}

fn step_1() -> i32 {
    let calories_records = read_input("input.txt");
    let calories_per_elves = calculate_calories_per_elves(&calories_records);

    max_calories(&calories_per_elves)
}

fn sum_max_three_calories(calories_per_elves: &Vec<i32>) -> i32 {
    calories_per_elves.iter().rev().take(3).sum()
}

fn step_2() -> i32 {
    let calories_records = read_input("input.txt");
    let mut calories_per_elves = calculate_calories_per_elves(&calories_records);
    calories_per_elves.sort();

    sum_max_three_calories(&calories_per_elves)
}

fn main() {
    assert_eq!(step_1(), 69912);

    assert_eq!(step_2(), 208180);
}
