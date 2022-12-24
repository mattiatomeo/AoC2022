use command::CommandExecution;


mod command;
mod history_parser;


fn step_1(history: &Vec<CommandExecution>) -> i64 {
    0
}

fn main() {
    println!("{:#?}", history_parser::infer_filesystem_from_history("input_test.txt"));
}
