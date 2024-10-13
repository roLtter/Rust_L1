use std::collections::HashSet;

fn main(){
    let set1:HashSet<i32> = [1, 4, 6, 34, 7].iter().cloned().collect();
    let set2:HashSet<i32> = [1, 2, 3, 4, 5, 6, 7].iter().cloned().collect();

    let intersection:HashSet<_> = set1.intersection(&set2).cloned().collect();

    print!("Intersection:{:?}", intersection);
}