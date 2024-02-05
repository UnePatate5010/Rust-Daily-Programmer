use crate::exercises::reverse_factorial;

pub fn main_reverse_factorial() {
    print!("#286 [EASY] reverse_factorial\t");
    assert_eq!(reverse_factorial::reverse_factorial(150), false);
    assert_eq!(reverse_factorial::reverse_factorial(120), true);
    println!("OK");
}