use colored::*;

mod utils;

struct Arg{
    pattern: String,
    path: String,
    ignore_case: bool,
    is_dir: bool,
    regex: bool
}

fn main() {
    let len = std::env::args().len();
    let mut arg: Arg = Arg{
        pattern: std::env::args().nth(len-2).expect("no pattern given"),
        path: std::env::args().nth(len-1).expect("no path given"),
        ignore_case: false,
        is_dir: false,
        regex: false
    };

    for i in 1..len{
        let cur = std::env::args().nth(i).unwrap();
        match cur.as_str() {
            "-i" | "--ignore-case" => arg.ignore_case = true,
            "-d" | "--dir" => arg.is_dir = true,
            "-r" | "--regex" => arg.regex = true,
            "-h" | "--help" => { utils::help::print_help(); return},
            _ => ()
        }
    }

    // Execution Time start
    let time_start = std::time::SystemTime::now();

    if arg.is_dir{
        visit_dirs(&arg.path, &arg).expect("Invalid path");
    }
    else{
        search_file(&arg.path, &arg);
    }

    print!("\nExecuted in {}ms\n", time_start.elapsed().unwrap().as_millis());
}

fn visit_dirs(dir: &str, arg:&Arg) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dirs(&path.to_str().unwrap(), arg)?;
        } else {
            search_file(&path.to_str().unwrap(), arg);
        }
    }
    Ok(())
}

fn search_file(path:&str, arg:&Arg){
    let contents = match std::fs::read_to_string(path){
        Ok(v)=>v,
        _ => return
    };
    let arr: Vec<&str> = contents.split('\n').collect();
    let mut res = String::new();
    let mut check = false;

    for i in 0..arr.len(){
        if utils::is_present(&arg.pattern, arr[i], arg.ignore_case, arg.regex) {
            check = true;
            res.push_str(&format!("{} {}\n", (i+1).to_string().yellow(), arr[i]));
        }
    }
    if check {
        print!("\n[{}]\n{}",get_filepath(path).blue(), res) ;
    }
}

fn get_filepath(path:&str)->&str{
    path
}