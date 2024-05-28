use crate::exercises::abacaba_sequence;

pub fn main_pancake_sort() {
    print!("#391 [EASY] ABACABA sequence\t");
    assert_eq!(abacaba_sequence::sequence(2), "aba".to_string());
    assert_eq!(abacaba_sequence::sequence(3), "abacaba".to_string());
    assert_eq!(abacaba_sequence::sequence(4), "abacabadabacaba".to_string());
    assert_eq!(abacaba_sequence::sequence(5), "abacabadabacabaeabacabadabacaba".to_string());
    println!("OK");
}