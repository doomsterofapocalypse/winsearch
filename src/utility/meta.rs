use crate::utility::util::enumerate_files;
use std::{path::PathBuf, time::{Duration}};

extern crate bytesize;
use bytesize::ByteSize;


pub fn greater_than(root_dirs: &Vec<PathBuf>, root_files:&Vec<PathBuf>, size_to_check:ByteSize){
    for dir in root_dirs{
        let entries = enumerate_files(dir);
        for file in entries{
            match file.metadata(){
                Ok(md) => {
                    let size = ByteSize(md.len());
                    if size > size_to_check{
                        println!("file: {:?} Size: {:?}", file.path(), size)
                    }
                },
                Err(e) => {
                    println!("Access error for file: {:?} Error: {}", file, e)
                },
            }
        }
        //println!("checking file: {:?}", file);
        
    }

    for file in root_files{
        match file.metadata(){
            Ok(md) => {
                let size = ByteSize(md.len());
                if size > size_to_check{
                    println!("file: {:?} Size: {:?}", file.as_path(), size)
                }
            },
            Err(e) => {
                println!("Access error for file: {:?} Error: {}", file, e)
            },
        }
    }

}


pub fn modfiles(root_dirs: &Vec<PathBuf>, qtime:u64, modified:bool, accessed:bool){
    for dir in root_dirs{
        let entries = enumerate_files(dir);
        for file in entries{
            match file.metadata(){
                Ok(md) => {
                    if modified{
                        if let Ok(x) = md.modified(){
                            let last_modified = x.elapsed().unwrap();
                            let check_time = Duration::from_secs(qtime);
                            if last_modified < check_time{
                                println!("file modified in the last {:?} secs is {:?}",check_time, file.path() );
                            }
                        }
                    }

                    if accessed{
                        if let Ok(x) = md.accessed(){
                            let last_modified = x.elapsed().unwrap();
                            let check_time = Duration::from_secs(qtime);
                            if last_modified < check_time{
                                println!("file was accessed in the last {:?} secs is {:?}",check_time, file.path() );
                            }
                        }
                    }
                    
                },
                Err(e) => {
                    println!("Access error for file: {:?} Error: {}", file, e)
                },
                
            }
        }
    }
}