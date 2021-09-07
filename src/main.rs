mod utils;

struct Arg{
    pattern: String,
    path: String,
    ignore_case: bool
}

fn main() {
    let len = std::env::args().len();
    let mut arg: Arg = Arg{
        pattern: std::env::args().nth(len-2).expect("no pattern given"),
        path: std::env::args().nth(len-1).expect("no path given"),
        ignore_case: false
    };

    for i in 1..len{
        let cur = std::env::args().nth(i).unwrap();
        match cur.as_str() {
            "-i" | "--ignore-case" => arg.ignore_case = true,
            "-h" | "--help" => { utils::help::print_help(); return},
            _ => ()
        }
    }

    // Searching single file
    let time_start = std::time::SystemTime::now();

    let contents = std::fs::read_to_string(arg.path).expect("Invalid path");
    let arr: Vec<&str> = contents.split('\n').collect();

    for i in 0..arr.len(){
        if utils::is_present(&arg.pattern, arr[i], arg.ignore_case) {
            print!("[{}] {}\n", i+1, arr[i]);
        }
    }

    print!("\nExecuted in {}ms\n", time_start.elapsed().unwrap().as_millis());
}

