use std::fs;
use std::cell::RefCell;
use std::io::{Error, ErrorKind};
use std::rc::Rc;
use scanf::sscanf;
use day_7::{Directory, File};

fn main() {
    let path = std::env::args().skip(1).next().expect("path not provided");
    let root = parse_input(path).expect("could not parse input");

    let required_space: u32 = 30000000;
    let disk_space = 70000000;

    println!("root: {:#?}", root);

    let used_space = Directory::total_size(&*root.borrow());

    let space_lacking = required_space - (disk_space - used_space as u32);

    println!("{}", &space_lacking);

    let directory_to_be_deleted = Directory::find_subdirectories(
        root, |d| d.borrow().total_size() >= space_lacking as usize);
    let directory_to_be_deleted = directory_to_be_deleted
        .iter().min_by_key(|d| d.borrow().total_size()).unwrap();

    println!("Directory that could be deleted: {:#?}", directory_to_be_deleted);
    println!("This would free up {} bytes of space.", Directory::total_size(&*directory_to_be_deleted.borrow()));

    fn parse_input(path: String) -> Result<Rc<RefCell<Directory>>, Error> {
        let root = Rc::new(RefCell::new(Directory::new(String::from("root"), None)));

        let mut current_dir = Rc::clone(&root);

        let s = fs::read_to_string(path)?;

        for command in s.replace("\r", "").split("$ ").skip(2) {
            let command = command.trim_end_matches("\n");

            match command {
                ls_cmd if command.starts_with("ls") => {
                    parse_ls(ls_cmd, Rc::clone(&current_dir))?;
                }
                cd_cmd if command.starts_with("cd") => {
                    if cd_cmd.starts_with("cd ..") {
                        let p = current_dir.borrow()
                            .parent()
                            .ok_or(Error::new(ErrorKind::InvalidData, "parent does not exist"))?
                            .clone();
                        current_dir = p.upgrade().unwrap();
                    } else {
                        let subdir_name = cd_cmd.trim_start_matches("cd ");
                        let d = current_dir.borrow().get_subdir(subdir_name)
                            .ok_or(Error::new(ErrorKind::InvalidData, "subdirectory does not exist"))?
                            .clone();
                        current_dir = d;
                    }
                }
                _ => { return Err(Error::new(ErrorKind::InvalidData, "unrecognized command")); }
            }
        }

        Ok(Rc::clone(&root))
    }

    fn parse_ls(ls_cmd: &str, current_dir: Rc<RefCell<Directory>>) -> Result<(), Error> {
        for line in ls_cmd.split("\n").skip(1) {
            if line.starts_with("dir ") {
                let name = line.trim_start_matches("dir ");
                if current_dir.borrow().contains_dir(name) { continue; }
                current_dir.borrow_mut().add_dir(RefCell::new(Directory::new(
                    name.to_owned(),
                    Some(Rc::downgrade(&current_dir)),
                )));
            } else if line.starts_with(char::is_numeric) {
                let mut size: u32 = 0;
                let mut name = String::new();
                sscanf!(line, "{u32} {string}", size, name)?;
                let file = File::new(name, size as usize);
                if current_dir.borrow().contains_file(&file) { continue; }
                current_dir.borrow_mut().add_file(file);
            }
        }

        Ok(())
    }
}