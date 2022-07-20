extern crate regex;
extern crate walkdir;

use crate::utility::util::enumerate_files;
use walkdir::{WalkDir, DirEntry};
use std::{path::{PathBuf}, io::BufRead, io::BufReader, fs::File};
use regex::Regex;


pub fn search_text(root_dirs: &Vec<PathBuf>,  query:&str) {
    let re = Regex::new(query).unwrap();
    for dir in root_dirs{
        let entries = enumerate_files(dir);
        for filename in entries{
            let filename = filename.path();
            let file = File::open(filename).unwrap();
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate(){
                match line{
                    Ok(line) => {
                        if re.is_match(&line){
                            let line: String = line.trim().chars().take(50).collect();
                            println!("Match found in: {:?}", filename);
                            print!("line no: {0: <6} =>", index.to_string());
                            println!("{}", line);
                            println!("");
                        }
                    },
                    //Err(e) => println!("Error reading file: {:?} due to error: {}", filename, e),
                    Err(_e) => (),            
                    
                }
    
            }
        }

    }
}


pub fn find_file(root_dirs: &Vec<PathBuf>, query: &str, ext: &str, inv:bool){
    //let re = Regex::new(query).unwrap();

    for dir in root_dirs{
        //println!("parent directory: {:?}", &dir);
        let entries:Vec<_> = WalkDir::new(&dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

        if inv{
            if inverse(entries, query, ext){
                println!("File not found in directory {:?}", &dir); 
            }
        }

        else{
            for filename in entries{
                let fullpath = &filename;
                let filename = filename.file_name().to_str();
                match filename{
                    Some(x) => {
                        if x.to_ascii_lowercase().contains(query) 
                        && x.to_ascii_lowercase().ends_with(ext){
                            println!("found file {} in directory {:?}", &x, &dir); 
                            if let Some(y) = fullpath.path().to_str(){
                                println!("Exact path: {:?}", &y);
                                println!("");
                            }
                                 
                        }
                    },
                    None => ()
    
                }
            }
        }
    }
    
}

fn inverse(entries:Vec<DirEntry>, query:&str, ext: &str) -> bool {
    
    let mut file_count = 0;
    for filename in entries{
        let filename = filename.file_name().to_str();
        match filename{
            Some(x) => {
                if x.to_ascii_lowercase().contains(query) && x.to_ascii_lowercase().ends_with(ext){
                    file_count+=1;      
                }
            },
            None => ()

            }
        }

        if file_count != 0{
            return false;
        }

        else {
            return true;
        }
        
}