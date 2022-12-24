use std::{collections::HashMap, hash::Hash};

use regex::Regex;

use crate::command::{CommandExecution, ObjectType};

struct DirectorySize {
    subdirectories: Vec<String>,
    file_size: u32,
}

impl DirectorySize {
    fn from_ls_command_output(ls_output: &CommandExecution) -> DirectorySize {
        let mut total_single_file_size: u32 = 0;
        let mut subdirectories: Vec<String> = Vec::new();

        for fs_object in &ls_output.output {
            match fs_object.object_type {
                ObjectType::File => total_single_file_size += fs_object.size,
                ObjectType::Dir => subdirectories.push(fs_object.name.clone()),
            };
        }

        DirectorySize { subdirectories, file_size: total_single_file_size }
    }
}

#[derive(Debug)]
struct DirectorySizeDiscoverer {
    subdirectories: HashMap<String, Vec<String>>,
    directory_size: HashMap<String, i32>,
    directory_traversal_history: Vec<String>,
    current_directory: String,
}

impl DirectorySizeDiscoverer {
    pub fn new() -> DirectorySizeDiscoverer {
        DirectorySizeDiscoverer {
            subdirectories: HashMap::new(),
            directory_size: HashMap::new(),
            directory_traversal_history: Vec::new(),
            current_directory: "".to_string(),
        }
    }

    pub fn discover_dir_sizes(&self, history: &Vec<CommandExecution>) -> HashMap<String, i32> {
        self.directory_size.clone()
    }

    fn parse_history(&self, history: &Vec<CommandExecution>) {
        let history_iter = history.iter();
        let root_folder = get_directory_from_change_dir_command(&history_iter.next().unwrap());

        self.change_current_folder(root_folder);

        for command in history_iter {
            if is_change_dir_command(command) {
                self.parse_cd_command(command);
            } else {
                self.parse_ls_command(command);
            }
        }
    }

    fn change_current_folder(&self, new_current: String) {
        self.directory_traversal_history.push(new_current);
        self.current_directory = new_current;
    }

    fn parse_cd_command(&self, command: &CommandExecution) {
        let cd_back_command = String::from("cd ..");

        match command.command {
            cd_back_command => self.move_one_level_above(),
            _ => self.move_one_level_below(get_directory_from_change_dir_command(command)),
        }
    }
    fn move_one_level_above(&self) {
        self.current_directory = self.directory_traversal_history.pop().unwrap();
    }

    fn move_one_level_below(&self, directory: String) {
        if ! self.is_curr_directory_already_inspectioned_with_ls() {
            self.directory_size.insert(directory, -1);
        }

        self.change_current_folder(directory);
    }

    fn parse_ls_command(&self, output: &CommandExecution) {
        if self.is_curr_directory_already_inspectioned_with_ls() {
            return
        }

        let mut directory_size: i32 = 0;
        let mut subdirectories: Vec<String> = Vec::new();
        for obj in output.output {

            match obj.object_type {
                ObjectType::File => directory_size += obj.size,
                ObjectType::Dir => subdirectories.push(obj.name),
            }
        }

        self.subdirectories.insert(self.current_directory, subdirectories);
        self.directory_size.insert(self.current_directory, directory_size);
    }

    fn is_curr_directory_already_inspectioned_with_ls(&self) -> bool {
        match self.directory_size.get(&self.current_directory) {
            Some(dir_size) => *dir_size != -1,
            None => false,
        }
    }
}

pub fn discover_dir_sizes(history: &Vec<CommandExecution>) -> HashMap<String, u32> {

}


fn is_change_dir_command(command: &CommandExecution) -> bool {
    let change_dir_regex = Regex::new(r"^cd .+$").unwrap();
    change_dir_regex.is_match(&command.command)
}

fn is_ls_command(command: &CommandExecution) -> bool {
    return command.command == "ls"
}

fn get_directory_from_change_dir_command(command: &CommandExecution) -> String {
    let change_dir_regex = Regex::new(r"^cd (.+)$").unwrap();

    let captures = match change_dir_regex.captures(&command.command) {
        Some(cap) => cap,
        None => panic!("Failed to parse command {}", command.command),
    };

    captures.get(1).map(|cap| String::from(cap.as_str())).unwrap()
}
