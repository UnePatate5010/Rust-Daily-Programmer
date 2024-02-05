mod t_roman_numeral_comparison;
mod t_caesar_cypher;
mod t_reverse_factorial;
mod t_next_palindrome;

pub fn main_tests() {
    t_reverse_factorial::main_reverse_factorial();
    t_caesar_cypher::main_caesar_cypher();
    t_next_palindrome::main_next_palindrome();
    t_roman_numeral_comparison::main_roman_numeral_comparison();
}