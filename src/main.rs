use std::env;
use std::path::Path;
use path_abs::PathAbs;
//use std::fs::File;
//use std::io::Read;

fn run_prompt(){
    // TOBE implemented
    println!("implement prompt")
}
fn run(fpath: &Path) {
    // TOBE implemented
    println!("process this file : {:?}", fpath)
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
                run(&filepath)
            }
        },
        _ => println!("currently supports only 1 file for parsing and running"),
    }
}

