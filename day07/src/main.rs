extern crate core;

use std::error::Error;
use std::fmt::format;
use std::fs;
use std::ops::Not;
use std::os::unix::raw::uid_t;
use std::rc::Rc;

struct File {
    name: String,
    size: u64,
}

struct Dir {
    files: Vec<File>,
    dirs: Vec<Dir>,
    name: String,
}


struct FileTree {
    root: Dir,
}


impl FileTree {
    fn new() -> FileTree {
        FileTree {
            root: Dir::new("/".to_string())
        }
    }

    fn find_dir(&mut self, name: &str) -> Option<&mut Dir> {
        println!("find_dir: {}", name);

        if name == "/" {
            return Some(&mut self.root);
        }

        self.root.find_dir(name[1..].as_ref())
    }
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn find_dir(&mut self, name: &str) -> Option<&mut Dir> {
        if !name.contains('/') {
            return self.dirs.iter_mut().find(|dir| dir.name == name.to_string())
        }

        match name.split_terminator('/').collect::<Vec<&str>>().as_slice() {
            [] => None,
            [n] => self.dirs.iter_mut().find(|dir| dir.name == n.to_string()),
            [n, rest @ ..] => {
                match self.dirs.iter_mut().find(|dir| dir.name == n.to_string()) {
                    Some(dir) => dir.find_dir(rest.join("/").as_ref()),
                    None => None
                }
            }
        }
    }

    fn size(&self) -> u64 {
        self.files.iter().map(|file| file.size).sum::<u64>() +
            self.dirs.iter().map(|dir| dir.size()).sum::<u64>()
    }

    fn get_dir_and_subdirs(&self) -> Vec<&Dir> {
        let mut dirs = Vec::new();
        dirs.push(self);
        for dir in self.dirs.iter() {
            dirs.append(&mut dir.get_dir_and_subdirs());
        }
        dirs
    }
}

enum Input {
    ls(),
    cd(String),
    dir(String),
    file(String, u64),
}

fn create_file_tree() -> Result<FileTree, Box<dyn Error>> {
    let inputs = read_input("input.txt")?;

    let mut current_dir: String = "/".to_owned();
    let mut file_tree = FileTree::new();

    for i in inputs {
        match i {
            Input::ls() => {}
            Input::cd(dir) if dir == ".." => {
                let last_slash = current_dir.rfind(|x| x == '/').ok_or("expected to find /")?;
                current_dir = current_dir[..last_slash].to_owned();
            }
            Input::cd(dir) if dir == "/" => {
                current_dir = "/".to_owned();
            }
            Input::cd(dir) if current_dir == "/" => {
                current_dir = format!("/{}", dir);
            }
            Input::cd(dir) => {
                current_dir = format!("{}/{}", current_dir, dir);
            }
            Input::dir(name) if current_dir == "/" => {
                file_tree.root.dirs.push(Dir::new(name));
            }
            Input::dir(name) => {
                file_tree.find_dir(current_dir.as_ref())
                    .ok_or(format!("expected to find dir {}", name ))?
                    .dirs.push(Dir::new(name));
            }
            Input::file(name, size) => {
                file_tree.find_dir(current_dir.as_ref())
                    .ok_or(format!("expected to find dir {} for file {}", current_dir, name))?
                    .files.push(File { name, size });
            }
        }
    }
    Ok(file_tree)
}


fn main() -> Result<(), Box<dyn Error>> {

    let file_tree = create_file_tree()?;

    let part1 = file_tree.root.get_dir_and_subdirs().iter().filter(|x|x.size() <= 100000).map(|x|x.size()).sum::<u64>();
    println!("{}", part1);



    let total_size = file_tree.root.size();
    let size_needed = 30000000 - (70000000 - total_size);

    let part2 = file_tree.root
        .get_dir_and_subdirs()
        .iter()
        .filter(|x|x.size() > size_needed)
        .map(|x|x.size())
        .min()
        .ok_or("min value expected")?;
    println!("{}", part2);

    Ok(())
}



fn read_input(file: &str) -> Result<Vec<Input>, Box<dyn Error>> {
    let mut inputs = Vec::new();
    let input = fs::read_to_string(file)?;
    for i in input.split('\n') {
        let input = match i.split(' ').collect::<Vec<&str>>()[..] {
            ["$","cd", d] => Ok(Input::cd(d.to_string())),
            ["$","ls"] => Ok(Input::ls()),
            ["dir", d] => Ok(Input::dir(d.to_string())),
            [s,f,..] => Ok(Input::file(f.to_string(), s.parse::<u64>().unwrap())),
            _ => Err(format!("Invalid input: {i}")),
        };
        inputs.push(input?);
    }
    Ok(inputs)
}
