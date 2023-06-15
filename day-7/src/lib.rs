use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Directory {
    name: String,
    parent: Option<Weak<RefCell<Directory>>>,
    children: HashMap<String, Rc<RefCell<Directory>>>,
    files: Vec<File>
}

#[derive(Debug, Eq, PartialEq)]
pub struct File {
    name: String,
    size: usize,
}

impl Directory {
    pub fn total_size(&self) -> usize {
        let mut res = 0;

        res += self.files.iter().map(|f|f.size).sum::<usize>();
        res += self.children.values().map(|c| Directory::total_size(&*c.borrow())).sum::<usize>();

        res
    }

    pub fn find_subdirectories<F>(root: Rc<RefCell<Directory>>, predicate: F) -> Vec<Rc<RefCell<Directory>>>
        where
            F: Fn(Rc<RefCell<Directory>>) -> bool,
    {
        let mut result = Vec::new();

        fn search_directory<F>(
            directory: Rc<RefCell<Directory>>,
            predicate: &F,
            result: &mut Vec<Rc<RefCell<Directory>>>,
        )
            where
                F: Fn(Rc<RefCell<Directory>>) -> bool,
        {
            if predicate(Rc::clone(&directory)) {
                result.push(directory.clone());
            }

            let subdirectories = directory.borrow().children.values().cloned().collect::<Vec<_>>();
            for subdir in subdirectories {
                search_directory(subdir, predicate, result);
            }
        }

        search_directory(root, &predicate, &mut result);
        result
    }

    pub fn new(name: String, parent: Option<Weak<RefCell<Directory>>>) -> Self {
        Self { name, parent, children: HashMap::new(), files: Vec::new() }
    }
    pub fn parent(&self) -> Option<Weak<RefCell<Directory>>> {
        match &self.parent {
            None => None,
            Some(v) => Some(v.clone())
        }
    }

    pub fn get_subdir(&self, name: &str) -> Option<&Rc<RefCell<Directory>>> {
        self.children.get(name)
    }

    pub fn contains_dir(&self, name: &str) -> bool {
        self.children.contains_key(name)
    }

    pub fn add_dir(&mut self, subdir: RefCell<Directory>) {
        let name = subdir.borrow().name.clone();
        self.children.insert(name, Rc::new(subdir));
    }

    pub fn add_file(&mut self, f: File) {
        self.files.push(f);
    }

    pub fn contains_file(&self, f: &File) -> bool {
        self.files.contains(f)
    }
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}