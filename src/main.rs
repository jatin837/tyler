use std::env;
use std::path::Path;
use path_abs::PathAbs;
//use std::fs::File;
//use std::io::Read;

fn run_prompt(){
    // TOBE implemented
    println!("implement prompt")
}
fn run_file(fpath: &Path) {
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
                run_file(&filepath)
            }
        },
        _ => eprintln!("[USAGE] tyler <file_name> (for script), tyler (for running prompt)"),
    }
}

