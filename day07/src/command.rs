use std::fs;


#[derive(Debug)]
pub struct CommandExecution {
    pub command: String,
    pub output: Vec<FileSystemObject>,
}

#[derive(Debug)]
pub enum ObjectType {
    File,
    Dir,
}

#[derive(Debug)]
pub struct FileSystemObject {
    pub name: String,
    pub object_type: ObjectType,
    pub size: i32,
}

impl FileSystemObject {
    pub fn from_output(output: Vec<String>) -> Vec<FileSystemObject> {
        output.iter()
            .filter(|row| row.len() > 0)
            .map(FileSystemObject::split_output_row_in_two)
            .map(|split_result| FileSystemObject::new(split_result.0, split_result.1))
            .collect()
    }

    fn new(object_type_or_size: &str, name: &str) -> FileSystemObject {
        match object_type_or_size {
            "dir" => FileSystemObject { name: String::from(name), object_type: ObjectType::Dir, size: -1 },
            _ => FileSystemObject { name: String::from(name), object_type: ObjectType::File, size: object_type_or_size.parse::<i32>().unwrap() },
        }
    }
    fn split_output_row_in_two(output_row: &String) -> (&str, &str) {
        let split: Vec<&str> = output_row.split(" ").collect();

        ( split[0], split[1] )
    }
}
impl CommandExecution {
    pub fn from_string(chunk: &str) -> CommandExecution {
        let rows: Vec<&str> = chunk.split('\n').collect();

        let command = String::from(rows[0].trim());
        let output: Vec<String> = rows[1..].iter()
            .map(|output_row| String::from(*output_row))
            .collect();


        CommandExecution { command: command, output: FileSystemObject::from_output(output) }
    }
}

pub fn read_history(filename: &str) -> Vec<CommandExecution> {
    fs::read_to_string(filename)
        .expect("Cannot read the input")
        .split('$')
        .map(CommandExecution::from_string)
        .collect()
}