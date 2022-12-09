use std::fs::{self, File};

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let test_file = ElFileSystemFile {name : String::from("2137.pap"), size: 2137};
    println!("test_file: {}", test_file.get_info());
}

struct ElFileSystemFile {
    name: String,
    size: usize
}

struct ElFileSystemDirectory {
    name: String,
    subdirectories: Vec<ElFileSystemDirectory>,
    files: Vec<ElFileSystemFile>
}

impl ElFileSystemFile {
    // fn new(&mut self, name: &str, size: usize) {
    //     self.name = String::from(name);
    //     self.size = size;
    // }
    fn get_info(&self) -> String {
        return format!("name: {}, size: {}", self.name, self.size);
    }
}