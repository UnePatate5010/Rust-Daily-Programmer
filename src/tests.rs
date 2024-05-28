mod t_roman_numeral_comparison;
mod t_caesar_cypher;
mod t_reverse_factorial;
mod t_next_palindrome;
mod t_fibonacci_ish_sequence;
mod t_letter_sum_value;
mod t_pancake_sort;
mod t_abacaba_sequencce;

pub fn main_tests() {
    t_fibonacci_ish_sequence::main_fibonacci_ish_sequence(); // 236
    t_reverse_factorial::main_reverse_factorial(); // 286
    t_caesar_cypher::main_caesar_cypher(); // 387
    t_next_palindrome::main_next_palindrome(); // 388
    t_abacaba_sequencce::main_pancake_sort();
    t_pancake_sort::main_pancake_sort(); // 392
    t_roman_numeral_comparison::main_roman_numeral_comparison(); // 397
    t_letter_sum_value::main_letter_sum_value(); // 399
}