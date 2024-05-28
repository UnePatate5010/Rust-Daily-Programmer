use crate::exercises::pancake_sort;


pub fn main_pancake_sort() {
    print!("#392 [INTERMEDIATE] Pancake sort\t");
    let arr = vec![0, 1, 2, 3, 4];
    assert_eq!(pancake_sort::pancake_sort(&arr, 2), vec![1, 0, 2, 3, 4]);
    assert_eq!(pancake_sort::pancake_sort(&arr, 5), vec![4, 3, 2, 1, 0]);
    assert_eq!(pancake_sort::pancake_sort(&vec![1, 2, 2, 2], 3), vec![2, 2, 1, 2]);
    println!("OK");
}