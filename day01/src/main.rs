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

fn step_1() -> i32 {
    let step_1_input = read_input("step_1.txt");

    let elves_meals = split_meals_by_elves(&step_1_input);

    let calories_per_elves = calculate_total_calories_per_elves(&elves_meals);

    max_calories(&calories_per_elves)
}

fn main() {
    assert_eq!(step_1(), 69912);
}
