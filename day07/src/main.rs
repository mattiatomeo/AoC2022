use std::collections::HashMap;

mod history_parser;

use history_parser::Directory;


fn get_directory_size_if_under_threshold(filesystem: &HashMap<String, Directory>, directory: &Directory, threshold: u64) -> Option<u64> {
    let mut dir_size = directory.files.iter().map(|file| file.size).sum::<u64>();

    if dir_size > threshold {
        return None;
    }

    for subdir in &directory.subdirectories {
        let subdir_obj = filesystem.get(subdir).unwrap();
        match get_directory_size_if_under_threshold(filesystem, subdir_obj, threshold) {
            Some(subdir_size) => dir_size += subdir_size,
            None => return None
        }

        if dir_size > threshold {
            return None;
        }
    }

    Some(dir_size)
}

fn step_1(filesystem: &HashMap<String, Directory>) -> u64 {
    let threshold = 100000;

    filesystem.values()
        .map(|directory| get_directory_size_if_under_threshold(filesystem, directory, threshold))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .sum()
}

fn main() {
    let filesystem = history_parser::infer_filesystem_from_history("input_test.txt");
    println!("{:#?}", step_1(&filesystem));
}
