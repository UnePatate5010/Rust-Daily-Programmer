use crate::exercises::fibonacci_ish_sequence;

pub fn main_fibonacci_ish_sequence() {
    print!("#236 [INTERMEDIATE] Fibonacci-ish sequence\t");
    assert_eq!(fibonacci_ish_sequence::fibo_sequence(21), 1);
    assert_eq!(fibonacci_ish_sequence::fibo_sequence(9), 3);
    assert_eq!(fibonacci_ish_sequence::fibo_sequence_2(21), 1);
    assert_eq!(fibonacci_ish_sequence::fibo_sequence_2(9), 3);    
    println!("OK");
}