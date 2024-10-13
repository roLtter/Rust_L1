fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let i = 2;

    if i < vec.len() {
        vec.remove(i);
    } else {
        println!("Индекс выходит за пределы вектора.");
    }

    println!("Вектор после удаления: {:?}", vec);
}
