use std::{collections::HashMap, fs};

use regex::Regex;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: u64,
}

impl Clone for File {
    fn clone(&self) -> File {
        File { name: self.name.clone(), size: self.size }
    }
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub subdirectories: Vec<String>,
    pub files: Vec<File>,
}

impl Directory {
    fn new(name: &String) -> Directory {
        Directory { name: String::from(name), subdirectories: Vec::new(), files: Vec::new() }
    }
}

impl Clone for Directory {
    fn clone(&self) -> Directory {
        Directory { name: self.name.clone(), subdirectories: self.subdirectories.clone(), files: self.files.clone() }
    }
}

struct CommandExecution {
    pub command: String,
    pub output: Vec<String>,
}

impl CommandExecution {
    fn from_string(command: String) -> CommandExecution {
        let rows: Vec<&str> = command.split('\n').collect();

        let command = String::from(rows[0].trim());
        let output: Vec<String> = rows[1..].iter()
            .map(|output_row| String::from(*output_row))
            .filter(|row| row.len() > 0)
            .collect();

        CommandExecution { command, output }
    }

    fn is_change_dir_command(&self) -> bool {
        let change_dir_regex = Regex::new(r"^cd (.+)$").unwrap();
        change_dir_regex.is_match(&self.command)
    }
}


struct FileSystemDiscoverer {
    directories: HashMap<String, Directory>,
    current_directory: String,
    directory_traversal_history: Vec<String>,
}

impl FileSystemDiscoverer {
    fn new() -> FileSystemDiscoverer {
        FileSystemDiscoverer { directories: HashMap::new(),
            current_directory: String::from("") ,
            directory_traversal_history: Vec::new(),
        }
    }

    fn explore_history(&mut self, commands: &Vec<CommandExecution>) {
        for command in commands {
            if command.is_change_dir_command() {
                self.parse_change_dir_command(command);
            } else {
                self.parse_ls_command(command);
            }
        }
    }

    fn parse_ls_command(&mut self, ls_command: &CommandExecution) {
        let mut current_directory = Directory::new(&self.current_directory);

        for row in &ls_command.output {
            FileSystemDiscoverer::update_directory_with_new_entry(&mut current_directory, row);
        }

        self.directories.insert(self.current_directory.clone(), current_directory);
    }

    fn update_directory_with_new_entry(directory: &mut Directory, row_from_ls: &String) {
        if row_from_ls.starts_with("dir") {
            let subdir = format!("{}/{}", directory.name, String::from(row_from_ls.split(' ').next_back().unwrap()));
            directory.subdirectories.push(subdir);
        } else {
            let file_definition: Vec<&str> = row_from_ls.split(' ').collect();
            directory.files.push(File { name: String::from(file_definition[1]), size: file_definition[0].parse::<u64>().unwrap() })
        }
    }

    fn parse_change_dir_command(&mut self, chdir_command: &CommandExecution) {
        match chdir_command.command.as_str() {
            "cd .." => self.go_back_to_parent(),
            _ => self.change_current_folder(FileSystemDiscoverer::get_directory_from_change_dir_command(chdir_command)),
        }
    }

    fn change_current_folder(&mut self, new_current_folder: String) {
        let new_current_folder_id = format!("{}/{}", self.current_directory, new_current_folder);

        match self.directories.get_mut(&self.current_directory) {
            Some(directory) => {
                if ! directory.subdirectories.contains(&new_current_folder_id) {
                    directory.subdirectories.push(new_current_folder_id.clone());
                }
            },
            None => {
                if self.current_directory != "" {
                    self.directories.insert(new_current_folder_id.clone(),Directory {
                        name: self.current_directory.clone(),
                        subdirectories: Vec::from_iter(vec![new_current_folder_id.clone()]),
                        files: Vec::new(),
                    });
                }
                ()
            }
        }

        self.directory_traversal_history.push(new_current_folder_id.clone());
        self.current_directory = String::from(new_current_folder_id.clone());
    }

    fn go_back_to_parent(&mut self) {
        let parent = self.get_parent();
        self.current_directory = parent;
    }

    fn get_parent(&mut self) -> String {
        match self.directory_traversal_history.pop() {
            Some(_) => self.directory_traversal_history.iter().next_back().unwrap().clone(),
            None => panic!("You are at the top of the hierachy"),
        }
    }

    fn get_discovered_folders(&self) -> HashMap<String, Directory> {
        self.directories.clone()
    }

    fn get_directory_from_change_dir_command(command: &CommandExecution) -> String {
        let change_dir_regex = Regex::new(r"^cd (.+)$").unwrap();

        let captures = match change_dir_regex.captures(&command.command) {
            Some(cap) => cap,
            None => panic!("Failed to parse command {}", command.command),
        };

        captures.get(1).map(|cap| String::from(cap.as_str())).unwrap()
    }
}

pub fn infer_filesystem_from_history(filename: &str) -> HashMap<String, Directory> {
    let history: Vec<CommandExecution> = fs::read_to_string(filename)
        .expect("Cannot read the input")
        .split('$')
        .filter(|row| row.len() > 0)
        .map(|row| CommandExecution::from_string(String::from(row)))
        .collect();

    let mut discoverer = FileSystemDiscoverer::new();

    discoverer.explore_history(&history);

    discoverer.get_discovered_folders()
}
