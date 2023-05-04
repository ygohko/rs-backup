use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::fs;
// use std::time::SystemTime;
use std::path::Path;
use std::path::PathBuf;

use filetime::{set_file_mtime, FileTime};

struct DirectoryEntries {
    path: String,
    file_paths: Vec<String>,
    directory_paths: Vec<String>,
}

struct BackUpExecuter {
    common_path: String,
}

impl BackUpExecuter {
    fn new () ->Self {
        return BackUpExecuter {
	    common_path: String::from(""),
	};
    }

    fn execute(&mut self, source_path: &String, destination_path: &String) -> std::io::Result<()> {

	self.initialize(source_path, destination_path);

	// TODO: Count same characters in paths
	/*
	let mut done = false;
	let index = 0;
	while !done {
	    if source_path[index] != destination_path[index] {
		done = true;
	    }
	}
	*/

	let mut entries_queue: VecDeque<DirectoryEntries> = VecDeque::new();
        let result = Self::get_directory_entries(source_path);
        if result.is_ok() {
            entries_queue.push_back(result.ok().unwrap());
        }

        let mut done = false;
        while !done {
            let entries = entries_queue.pop_front().unwrap();

            for directory_path in entries.directory_paths {
                let a_entries = Self::get_directory_entries(&directory_path)?;
                entries_queue.push_back(a_entries);
            }

            let mut directory_path_buf = PathBuf::new();
            directory_path_buf.push(destination_path.clone());
            directory_path_buf.push(entries.path);
            if !directory_path_buf.as_path().exists() {
                // Create destination directory

                // println!("path: {}", directory_path_buf.to_str().unwrap());

                std::fs::create_dir_all(directory_path_buf.as_path())?;
            }
            else {
                // Remove files that is not in the source directory
                let destination_entries = Self::get_directory_entries(&directory_path_buf.to_str().unwrap().to_string())?;
                for a_file_path in destination_entries.file_paths {
                    let mut path = a_file_path.clone();
                    path.replace_range(0..destination_path.len() + 1, "");
                    let mut found = false;
                    for b_file_path in entries.file_paths.clone() {
                        if b_file_path == path {
                            found = true;
                            break;
                        }
                    }

                    if !found {

                        // println!("Removing...");

                        std::fs::remove_file(Path::new(&a_file_path))?;
                    }
                }
            }

            for file_path in entries.file_paths {
                // Copy the file
                let mut file_destination_path_buf = PathBuf::new();
                file_destination_path_buf.push(destination_path.clone());
                file_destination_path_buf.push(file_path.clone());

                // println!("Copying from {} to {}...", file_path.clone(), file_destination_path_buf.to_str().unwrap().to_string());

                Self::copy(&file_path, &file_destination_path_buf.to_str().unwrap().to_string())?;

            }

            if entries_queue.len() <= 0 {
                done = true;
            }
        }

        return Ok(());
    }

    fn initialize(&mut self, source_path: &String, destination_path: &String) {
	let mut maximum_len = source_path.len();
	let destination_len = destination_path.len();
	if destination_len < maximum_len {
	    maximum_len = destination_len;
	}
	let mut common_len = 0;
	for i in 0..maximum_len {
	    if source_path.chars().nth(i) != destination_path.chars().nth(i) {
		break;
	    }
	    else {
		common_len += 1;
	    }
	}

	self.common_path = source_path.to_string();
	self.common_path.replace_range(common_len.., "");

	println!("self.common_path: {}", self.common_path);

    }

    fn copy(source_path: &String, destination_path: &String) -> std::io::Result<()> {
        let mut needed = true;
        if Path::new(&destination_path).exists() {
            let source_metadata = Path::new(&source_path).metadata()?;
            let source_modified = source_metadata.modified()?;
            let destination_metadata = Path::new(&destination_path).metadata()?;
            let destination_modified = destination_metadata.modified()?;

            /*
            let result = source_modified.duration_since(SystemTime::UNIX_EPOCH);
            let source_secs = result.ok().unwrap().as_secs();
            let result = destination_modified.duration_since(SystemTime::UNIX_EPOCH);
            let destination_secs = result.ok().unwrap().as_secs();
             */
            // println!("source: {}, destination: {}", source_secs, destination_secs);

            if source_modified.cmp(&destination_modified) != Ordering::Greater {
                needed = false;
            }
        }
        if needed {

            // println!("Copying...");

            std::fs::copy(Path::new(&source_path), Path::new(&destination_path))?;
            let source_metadata = Path::new(&source_path).metadata()?;
            set_file_mtime(destination_path, FileTime::from_last_modification_time(&source_metadata))?;
        }
        else {

            // println!("Skipping...");

        }

        return Ok(());
    }

    fn get_directory_entries(path: &String) -> Result<DirectoryEntries, std::io::Error> {
        let mut entries = DirectoryEntries {
            path: path.clone(),
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

        return Ok(entries);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("USAGE: rs-backup SOURCE DESTINATION");

        return Ok(());
    }
    let mut executer = BackUpExecuter::new();
    executer.execute(&args[1], &args[2])?;

    return Ok(());
}
