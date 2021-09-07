//struct Cli {
//    pattern: String,
//    path: std::path::PathBuf,
//}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    
    for i in 3..std::env::args().len(){
        //match std::env::args().nth(3) {
        //    Some(_v) => panic!("too many arguments"), 
        //    None => break, 
        //}
        print!("{}\n", std::env::args().nth(i).unwrap());
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

