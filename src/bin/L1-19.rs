fn reverse_words(input: &str) -> String{
    input.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

fn main() {
    let original = "snow dog sun";
    let reversed = reverse_words(&original);

    println!("Оригинальная строка: {}", original);
    println!("Перевернутые слова: {}", reversed);
}
