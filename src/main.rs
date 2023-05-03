use std::collections::VecDeque;
use std::env;
use std::fs;
// use std::path::Path;

struct DirectoryEntries {
    file_paths: Vec<String>,
    directory_paths: Vec<String>,
}

fn get_paths(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut paths: Vec<String> = vec![];
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let path = dir.path();
        let result = path.into_os_string().into_string();
        if result.is_ok() {
            paths.push(result.ok().unwrap());
        }
    }
    Ok(paths)
}

fn get_directory_entries(path: String) -> Result<DirectoryEntries, std::io::Error> {
    let mut entries = DirectoryEntries {
        file_paths: vec![],
        directory_paths: vec![],
    };
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let path = dir.path();
        let result = path.clone().into_os_string().into_string();
        if result.is_ok() {
            let path_string = result.ok().unwrap();
            if path.is_file() {
                entries.file_paths.push(path_string.clone());
            }
            if path.is_dir() {
                entries.directory_paths.push(path_string);
            }
        }
    }
    Ok(entries)
}

fn execute(source_path: String, destination_path: String) -> std::io::Result<()> {
    let mut entries_queue: VecDeque<DirectoryEntries> = VecDeque::new();
    let result = get_directory_entries(source_path);
    if result.is_ok() {
        entries_queue.push_back(result.ok().unwrap());
    }

    let mut done = false;
    while !done {
        let entries = entries_queue.pop_front().unwrap();

	for directory_path in entries.directory_paths {
            let a_entries = get_directory_entries(directory_path)?;
            entries_queue.push_back(a_entries);
        }

        for file_path in entries.file_paths {
            println!("Processing a file: {}", file_path);
        }

        if entries_queue.len() <= 0 {
            done = true;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    /*
    println!("Hello, world!");

    let args:Vec<String> = env::args().collect();
    let count = args.len();
    println!("count: {}", count);
    for i in 0..count {
        println!("arg: {}", args[i]);
    }

    for entry in fs::read_dir(".")? {
        let dir = entry?;
        let path = dir.path();
        println!("path: {}", path.display());
    }

    let paths = get_paths(".")?;
    for path in paths {
        println!("path: {}", path);
    }

    let entries = get_directory_entries(".".to_string())?;
    for file_path in entries.file_paths {
        println!("file_path: {}", file_path);
    }
    for directory_path in entries.directory_paths {
    println!("directory_path: {}", directory_path);
    }
     */

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
	println!("USAGE: rs-backup SOURCE DESTINATION");

	return Ok(());
    }
    execute(args[1].clone(), args[2].clone())?;

    return Ok(());
}
