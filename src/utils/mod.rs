use regex::Regex;
pub mod help;


pub fn is_present(pattern:&str, s: &str, case_sensitive: bool, is_regex:bool)-> bool{
    let arr: Vec<&str> = s.split(' ').collect();
    if is_regex{
        let re = Regex::new(pattern).expect("Invalid regex");
        for str in arr{
            if re.is_match(str){
                return true;
            }
        }
    }
    else {
        for str in arr{
            if str==pattern{
                return true;
            }
            else if case_sensitive && str.to_lowercase() == pattern.to_lowercase() {
                return true;
            }
        }
    }

    false
}

