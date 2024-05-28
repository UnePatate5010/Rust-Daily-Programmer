mod t_roman_numeral_comparison;
mod t_caesar_cypher;
mod t_reverse_factorial;
mod t_next_palindrome;
mod t_fibonacci_ish_sequence;
mod t_letter_sum_value;

pub fn main_tests() {
    t_fibonacci_ish_sequence::main_fibonacci_ish_sequence();
    t_reverse_factorial::main_reverse_factorial();
    t_caesar_cypher::main_caesar_cypher();
    t_next_palindrome::main_next_palindrome();
    t_letter_sum_value::main_letter_sum_value();
    t_roman_numeral_comparison::main_roman_numeral_comparison();
}