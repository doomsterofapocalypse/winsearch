extern crate clap;
extern crate bytesize;
mod utility;

use clap::Parser;
use utility::meta::*;
use utility::search::*;
use utility::util::*;
use bytesize::ByteSize;
use std::time::Instant;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cmd {
    ///Directory to start the search in
    #[clap(short, long)]
    path: Option<String>,
    ///Find files which have size greater than a specific number in mb <minimum value 1>
    #[clap(short, long)]
    greater: Option<u64>,
    ///Find files which have size less than a specific number in mb <minimum value 1>
    #[clap(short, long)]
    lesser: Option<u64>,
    ///Input file containing queries to search
    #[clap(short, long)]
    file: Option<String>,
    ///Search inside text of files
    #[clap(short, long)]
    text_search: bool,
    ///Search file names matching the query or regular expression
    #[clap(short, long)]
    name_search: bool,
    ///By default the search will report found folders, use inverse switch to report only folders where the query reported no finding
    #[clap(short, long)]
    inverse: bool,
    ///A word or regular expression to search
    #[clap(short, long)]
    query: Option<String>,
    ///Extension of the file e.g .txt (to be used in conjunction with name_search to find a file ending with said extension)
    #[clap(short, long)]
    extension: Option<String>,
    ///Duration mentioned in hh:mm:sec format used with options modified or accessed to get files changed within the given duration
    #[clap(short, long)]
    duration: Option<String>,
    ///Find files modified 
    #[clap(short, long)]
    modified: bool,
    ///Find accessed files
    #[clap(short, long)]
    accessed: bool,
}


fn main() {
    let start = Instant::now();

    let args = Cmd::parse();

    //this loop will work only if path is provided (some value is present)
    if let Some(path) = args.path.as_deref(){
        let (root_dirs, root_files) = enumerate_dirs(&path);
        //let files = enumerate_files(&root_dirs);
        match args.greater{
            Some(x) => {
                let size_to_check = ByteSize::mb(x);
                println!("size to check is: {:?}", size_to_check);
                greater_than(&root_dirs, &root_files, size_to_check)
    
                },
            None => (),
            }
        match args.file{
            Some(x) =>{
                let content = read_file(&x);
                for line in content{
                    search_text(&root_dirs, &line)
                }

            },
            None => {
                if let Some(x) = args.query{
                    if args.text_search{
                        search_text(&root_dirs, &x)
                    }
                    else if args.name_search{
                        let ext = args.extension.expect("Extension is required for searching");
                        if args.inverse{
                            find_file(&root_dirs, &x, &ext, true);
                        }
                        else{
                            find_file(&root_dirs,  &x, &ext, false);
                        }

                    }

                    else{
                    
                        panic!("usage: winsearch.exe -p <path> --options => either text search or name search need to be selected")
                    }
                    
                }
            }
        } 
        match args.duration{
            Some(x) => {
                let time = get_duration_sec(x);
                if args.modified{
                    modfiles(&root_dirs, time, true, false);
                }
                else if args.accessed{
                    modfiles(&root_dirs, time, false, true);
                }

                else{

                }
            }

            None => {
                
            }
        }
    }
    
    let duration = start.elapsed();
    println!("*******************************************************************");
    println!("Time taken to complete your query {:?}", duration);

}