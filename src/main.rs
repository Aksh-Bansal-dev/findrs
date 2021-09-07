mod help;

fn main() {
    let len = std::env::args().len();
    let pattern = std::env::args().nth(len-2).expect("no pattern given");
    let path = std::env::args().nth(len-1).expect("no path given");

    for i in 1..len{
        let arg = std::env::args().nth(i).unwrap();
        match arg.as_str() {
            "-i" | "--ignore-case" => print!("CASE INSENSITIVE\n"),
            "-h" | "--help" => { help::print_help(); return},
            _ => ()
        }
    }

    let contents = std::fs::read_to_string(path).expect("Invalid path");
    let arr: Vec<&str> = contents.split('\n').collect();

    for i in 0..arr.len(){
        if present(&pattern, arr[i], true) {
            print!("[{}] {}\n", i+1, arr[i]);
        }
    }
}

fn present(pattern:&str, s: &str, case_sensitive: bool)-> bool{
    let arr: Vec<&str> = s.split(' ').collect();

    for str in arr{
        if str==pattern{
            return true;
        }
        else if case_sensitive && str.to_lowercase() == pattern.to_lowercase() {
            return true;
        }
    }
    false
}

