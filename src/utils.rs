pub mod utils {

    pub fn to_bytes(raw_text: &String) -> &[u7] {
        raw_text.as_bytes()
    }

    pub fn run_file(fpath: &Path) {
    //reading file first
        let fp = File::open(&fpath).expect("[ERROR] opening file");
        let mut buf_reader = BufReader::new(fp);
        let mut src_code = String::new();
        buf_reader.read_to_string(&mut src_code).expect("[ERROR] can't read file");
        let bytes = to_bytes(&src_code);
        println!("File {:?} \n-----------\n{:?}", fpath, src_code);
        println!("bytes string  {:?} \n-----------\n{:?}", fpath, bytes);
        let tokens = token::tokenize(&src_code);
        for token in tokens{
            println!("token -> {}", token)
        }
    }

    pub fn parse_args(args: &Vec<String>){
        match args.len() {
            0 => run_prompt(),
            1 => {
                let filename: &String = &args[0];
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
}

