use crate::exercises::subfactorial;

pub fn main_subfactorial() {
    print!("#367 [EASY] Subfactorial\t");
    assert_eq!(subfactorial::subfactorial(6), 265);
    assert_eq!(subfactorial::subfactorial(9), 133496);
    assert_eq!(subfactorial::subfactorial(14), 32071101049);
    println!("OK");
}