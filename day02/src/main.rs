use std::{fs, collections::HashMap};


fn read_input(filename: &str) -> Vec<(char, char)> {
    let mut games: Vec<(char, char)> = Vec::new();

    let splitted: Vec<String> = fs::read_to_string(filename)
        .expect("Cannot read the input")
        .trim()
        .split('\n')
        .map(String::from)
        .collect();
    
    for game in splitted {
        let opponent_game = game.chars().nth(0).unwrap();
        let santa_game = game.chars().nth_back(0).unwrap();
        games.push((opponent_game, santa_game));
    }    
    games
}



fn play_rock_paper_scissor(your_move: &char, opponent_move: &char) -> u32 {
    let shape_score: HashMap<char, u32> = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3),
    ]);
    
    let shape_win: HashMap<char, char> = HashMap::from([
        ('X', 'C'),
        ('Y', 'A'),
        ('Z', 'B'),
    ]);

    let shape_draw: HashMap<char, char> = HashMap::from([
        ('X', 'A'),
        ('Y', 'B'),
        ('Z', 'C'),
    ]);

    static WIN_POINTS: u32 = 6;
    static DRAW_POINTS: u32 = 3;
    static LOOSE_POINTS: u32 = 0;

    let match_points = shape_score.get(your_move).expect("Unknow move");
    let opponent_move_for_win = shape_win.get(your_move).expect("Unknow move");
    let opponent_move_for_draw = shape_draw.get(your_move).expect("Unknow move");

    if opponent_move == opponent_move_for_win {
        match_points + WIN_POINTS
    } else if opponent_move == opponent_move_for_draw {
        match_points + DRAW_POINTS
    } else {
        match_points + LOOSE_POINTS
    }
}

fn play_following_the_elf_strategy(strategy: &char, opponent_move: &char) -> u32 {
    let shape_win: HashMap<char, char> = HashMap::from([
        ('C', 'X'),
        ('A', 'Y'),
        ('B', 'Z'),
    ]);

    let shape_draw: HashMap<char, char> = HashMap::from([
        ('A', 'X'),
        ('B', 'Y'),
        ('C', 'Z'),
    ]);

    let shape_loose: HashMap<char, char> = HashMap::from([
        ('A', 'Z'),
        ('B', 'X'),
        ('C', 'Y'),
    ]);
    
    match strategy {
        'Z' => play_rock_paper_scissor(&shape_win.get(opponent_move).unwrap(), opponent_move),
        'Y' => play_rock_paper_scissor(&shape_draw.get(opponent_move).unwrap(), opponent_move),
        _ => play_rock_paper_scissor(&shape_loose.get(opponent_move).unwrap(), opponent_move)
    }
}

fn step_1(games: &Vec<(char, char)>) -> u32 {
    games.iter().map(|game| play_rock_paper_scissor(&game.1, &game.0)).sum()
}

fn step_2(games: &Vec<(char, char)>) -> u32 {
    games.iter().map(|game| play_following_the_elf_strategy(&game.1, &game.0)).sum()
}

fn main() {
    let games = read_input("input.txt");

    assert_eq!(step_1(&games), 11841);
    assert_eq!(step_2(&games), 13022);
}
