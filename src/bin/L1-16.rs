fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = (low + high) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    None
}

fn main() {
    let arr = [-13, -5, -3, 2, 5, 5, 7, 9, 23, 32, 34, 62, 90];
    let target = 23;

    match binary_search(&arr, target) {
        Some(index) => println!("Элемент найден на позиции: {}", index),
        None => println!("Элемент не найден"),
    }
}
