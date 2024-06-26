mod t_roman_numeral_comparison;
mod t_caesar_cypher;
mod t_reverse_factorial;
mod t_next_palindrome;
mod t_fibonacci_ish_sequence;
mod t_letter_sum_value;
mod t_pancake_sort;
mod t_abacaba_sequencce;
mod t_yahtzee_scoring;
mod t_progressive_taxation;
mod t_julian_calendar;
mod t_subfactorial;
mod t_word_ladder;
mod t_iec_rule;

pub fn main_tests() {
    t_fibonacci_ish_sequence::main_fibonacci_ish_sequence(); // 236
    t_reverse_factorial::main_reverse_factorial(); // 286
    t_iec_rule::main_iec_rule(); // 363
    t_word_ladder::main_word_ladder(); // 363
    t_subfactorial::main_subfactorial(); // 367
    t_julian_calendar::main_julian_calendar(); // 376
    t_progressive_taxation::main_progressive_taxationt(); // 379
    t_yahtzee_scoring::main_yahtzee_scoring(); // 381
    t_caesar_cypher::main_caesar_cypher(); // 387
    t_next_palindrome::main_next_palindrome(); // 388
    t_abacaba_sequencce::main_pancake_sort();
    t_pancake_sort::main_pancake_sort(); // 392
    t_roman_numeral_comparison::main_roman_numeral_comparison(); // 397
    t_letter_sum_value::main_letter_sum_value(); // 399
}