use std::collections::HashMap;

mod history_parser;

use history_parser::Directory;

struct DirectorySizeCalculator {
    filesystem: HashMap<String, Directory>,
    directory_size: HashMap<String, u64>,
}

impl DirectorySizeCalculator {
    fn new(filesystem: &HashMap<String, Directory>) -> DirectorySizeCalculator {
        DirectorySizeCalculator { filesystem: filesystem.clone(), directory_size: HashMap::new() }
    }

    fn get_directories_size(&mut self) -> HashMap<String, u64> {
        self.calculate_directories_sizes();

        self.directory_size.clone()
    }

    fn calculate_directories_sizes(&mut self) {
        let mut directory_sizes: HashMap<String, u64> = HashMap::new();
        self.filesystem.values().for_each(|subdir| {
            self.calculate_directory_size_for_dir(subdir, &mut directory_sizes);
        });

        self.directory_size = directory_sizes;
    }

    fn calculate_directory_size_for_dir(&self, directory: &Directory, directory_size_cache: &mut HashMap<String, u64>) -> u64 {
        let mut dir_size = directory.files.iter().map(|file| file.size).sum::<u64>();

        for subdir in &directory.subdirectories {
            let subdir_obj = self.filesystem.get(subdir).unwrap();
            let subdir_size = match directory_size_cache.get(subdir) {
                Some(size) => *size,
                None => self.calculate_directory_size_for_dir(subdir_obj, directory_size_cache)
            };

            dir_size += subdir_size;
        }

        directory_size_cache.insert(directory.name.clone(), dir_size);

        dir_size
    }
}

fn step_1(filesystem: &HashMap<String, Directory>) -> u64 {
    let threshold: u64 = 100000;

    let mut dir_size_calculator = DirectorySizeCalculator::new(filesystem);
    dir_size_calculator.get_directories_size().values()
        .filter(|&&dir_size| dir_size <= threshold)
        .sum()
}

fn step_2(filesystem: &HashMap<String, Directory>) -> u64 {
    let disk_size: u64 = 70000000;
    let update_required_size: u64 = 30000000;

    let dir_sizes = DirectorySizeCalculator::new(filesystem).get_directories_size();

    let root_size = *dir_sizes.get("/").unwrap();
    let size_to_free = update_required_size - (disk_size - root_size);

    *dir_sizes.values().filter(|&&dir_size| dir_size > size_to_free).min().unwrap()
}
fn main() {
    let filesystem = history_parser::infer_filesystem_from_history("input.txt");
    assert_eq!(step_1(&filesystem), 1989474);
    assert_eq!(step_2(&filesystem), 1111607);
}
