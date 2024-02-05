use crate::exercises::next_palindrome;

pub fn main_next_palindrome() {
    print!("#388 [INTERMEDIATE] Next palindrome\t");
    assert_eq!(next_palindrome::next_palindrome(808), 818);
    assert_eq!(next_palindrome::next_palindrome(999), 1001);
    assert_eq!(next_palindrome::next_palindrome(2133), 2222);
    println!("OK");
}