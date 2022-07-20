extern crate regex;
extern crate walkdir;


use walkdir::{WalkDir, DirEntry};
use std::{path::{Path, PathBuf}, io::BufRead, io::BufReader, fs::File};



pub fn enumerate_dirs(path: &str) -> (Vec<PathBuf>,Vec<PathBuf> ){
    let rootpath = Path::new(path);
    let mut root_dirs:Vec<PathBuf> = vec![];
    let mut root_files:Vec<PathBuf> = vec![];
    for entry in rootpath.read_dir().expect("failed to read root directory"){
        if let Ok(entry) = entry{
            if entry.path().is_dir(){
                //println!("in enumerate_dirs and found Dir: {:?}", entry.path());
                root_dirs.push(entry.path())
            }
            else{
                root_files.push(entry.path())
            }
            
        }
    }
    
    (root_dirs, root_files)
}


pub fn enumerate_files(dir:&PathBuf) -> Vec<DirEntry>{
 
    let entries:Vec<_> = WalkDir::new(&dir)
         .into_iter()
         .filter_map(|e| e.ok())
         .filter(|e| e.file_type().is_file())
         .collect();
        
    return entries
    
}


pub fn read_file(path:&String)-> Vec<String>{
    let mut content:Vec<String> = vec![];
    let file = File::open(path).expect("Error opening this file");
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate(){
        match line{
            Ok(l) => content.push(l.trim().to_string()),
            Err(e) => println!("Error reading line: at index: {}, Error message: {}", index.to_string(), e )
        }
    }

    content
}

pub fn get_duration_sec(time:String) -> u64{

    let qtime:Vec<u64> = time.split(":")
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect();
    let total_sec = qtime[0]*3600+qtime[1]*60+qtime[2];
    total_sec

}