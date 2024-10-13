use std::collections::HashSet;

fn is_unique(s: &str) -> bool {
    let mut chars = HashSet::new();

    for c in s.chars() {
        let lower_c = c.to_ascii_lowercase();

        if !chars.insert(lower_c) {
            return false;
        }
    }
    true
}

fn main() {
    let test_str1 = "abcd";
    let test_str2 = "abCdefAaf";
    let test_str3 = "aabcd";

    println!("{} -> {}", test_str1, is_unique(test_str1));
    println!("{} -> {}", test_str2, is_unique(test_str2));
    println!("{} -> {}", test_str3, is_unique(test_str3));
}
