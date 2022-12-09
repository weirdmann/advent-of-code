#![feature(map_try_insert)]

use std::{fs::{self}, fmt, collections::HashMap, rc::Rc, cell::RefCell, borrow::Borrow};

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    //let mut home = ElFileSystemDirectory::new_home();
    let home = ElFileSystemDirectory::new_home();
    let test_file = ElFileSystemFile::new("2137.pap", 2137);

    let mut homeref = home.borrow_mut();
    homeref.file_new(test_file);
    match homeref.mkdir("papaj_drive1") {
        Ok(d) => {d.borrow_mut().mkdir("inside").unwrap();},
        Err(_) => {println!("error")},
    };
    homeref.mkdir("papaj_drive2");
    homeref.mkdir("papaj_drive3");
    homeref.mkdir("papaj_drive4");
    println!("{:?}", homeref);
}

struct ElFileSystemFile {
    name: String,
    size: usize
}

struct ElFileSystemDirectory {
    name: String,
    myref: Option<Rc<RefCell<ElFileSystemDirectory>>>,
    parent : Option<Rc<RefCell<ElFileSystemDirectory>>>,
    subdirectories: HashMap<String, Rc<RefCell<ElFileSystemDirectory>>>,
    files: HashMap<String, Rc<RefCell<ElFileSystemFile>>>
}

impl ElFileSystemFile {
    fn new(name: &str, size: usize) -> ElFileSystemFile {
         ElFileSystemFile{
            name : String::from(name),
            size : size
        }
    }
}

impl ElFileSystemDirectory {
    fn new(name : &str) -> Rc<RefCell<ElFileSystemDirectory>> {
        let dir = 
        Rc::new(RefCell::new(
        ElFileSystemDirectory { name: String::from(name),
            parent: None,
            myref: None,
            subdirectories: HashMap::new(),
            files: HashMap::new() 
        }));
        dir.borrow_mut().myref = Some(Rc::clone(&dir));
        dir
    }

    fn new_home() -> Rc<RefCell<ElFileSystemDirectory>> {
        ElFileSystemDirectory::new("/")
    }

    fn assign_parent(&mut self, parent : Rc<RefCell<ElFileSystemDirectory>>) {
        self.parent = Some(parent);
    }

    fn file_new(&mut self, file : ElFileSystemFile) {
        self.files.insert(
            String::from(&file.name), 
        Rc::new(RefCell::new(file))
    );
    }

    fn mkdir(&mut self, name : &str) -> Result<Rc<RefCell<ElFileSystemDirectory>>, String> {
        match self.subdirectories.get(name) {
            Some(_) => { Err(String::from("Directory already exists")) }
            None => {
                match self.subdirectories.try_insert(String::from(name), ElFileSystemDirectory::new(name)) {
                    Ok(d) => {d.borrow_mut().assign_parent(Rc::clone(&self.myref.as_ref().unwrap())); Ok(d.clone())},
                    Err(d) => {Err(String::from(format!("{:?}",d).as_str()))},
                }
            }
        }
    }

    fn get_tree(&self) -> String {
        let mut tree = String::new();
        for (_name, _subdirectory) in &self.subdirectories {
            tree = format!("{}\ndir {}", tree, _name);
            for (_n, _s) in &_subdirectory.borrow_mut().subdirectories {
                tree = format!("{}\t{}", tree, _s.borrow_mut().get_tree());
            }
        }
        for (_name, _file) in &self.files {
            tree = format!("{}\n{:?}", tree, _file.borrow_mut());
        }
        tree
    }

    fn get_absolute_path(&self) -> String {
        match &self.parent {
            Some(parent) => { 
                format!("{}/{}", parent.borrow_mut().get_absolute_path(), self.name) 
            }
            None => {
                String::from("/")
            }
        }
    }
}

impl fmt::Debug for ElFileSystemFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("{:>10} {}", self.size, self.name).as_str())
         .finish()
    }
}

impl fmt::Debug for ElFileSystemDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("{}:{}", self.get_absolute_path(), self.get_tree()).as_str())
         .finish()
    }
}