use std::env;
use std::path::Path;
use path_abs::PathAbs;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

//use std::io::Read;

fn run_prompt(){
    // TOBE implemented
    println!("implement prompt")
}
fn run_file(fpath: &Path) {
//reading file first
    let fp = File::open(&fpath).expect("[ERROR] opening file");
    let mut buf_reader = BufReader::new(fp);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("can't read file");
    print!("File {:?} \n-----------\n{:?}", fpath, contents)

}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => {
            let filename: &String = &args[1];
            let abs_file_path = PathAbs::new(filename).unwrap();
            let filepath:&Path = Path::new(&abs_file_path);
            if !filepath.exists() {
                eprintln!("[ERROR] provide a file that exists")
            }else {
                run_file(&filepath)
            }
        },
        _ => eprintln!("[USAGE] tyler <file_name> (for script), tyler (for running prompt)"),
    }
}

