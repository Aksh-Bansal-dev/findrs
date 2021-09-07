pub mod help;

pub fn is_present(pattern:&str, s: &str, case_sensitive: bool)-> bool{
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

