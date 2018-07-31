use std::io::Read;
use std::env;
use std::io;
use std::fs::File;
use std::process;
use std::path::Path;

mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("{:?}", args.len());
    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> () {
    println!("Running file {}", path);
    match read_file(&Path::new(path)) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => run(&s),
    }
}

fn run_prompt() {
    println!("I am the run prompt function");
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        run(&input);
    }
}

fn run(s: &str) {
    let scanner = scanner::Scanner::new();
}

fn read_file(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
