fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let original = "главрыба";
    let reversed = reverse_string(original);

    println!("Оригинальная строка: {}", original);
    println!("Перевернутая строка: {}", reversed);
}
