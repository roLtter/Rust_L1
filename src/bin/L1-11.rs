use std::collections::HashMap;

fn main() {
    let temperatures: Vec<f64> = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let mut ranges: HashMap<String, Vec<f64>> = HashMap::new();

    for &temp in &temperatures {
        let lower_bound = (temp / 10.0).floor() * 10.0;
        let upper_bound = lower_bound + 10.0;

        let range_key = format!("[{}, {}):", lower_bound, upper_bound);

        ranges.entry(range_key).or_insert(Vec::new()).push(temp);
    }

    for (range, temps) in &ranges {
        println!("{} {:?}", range, temps);
    }
}
