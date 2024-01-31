use crate::exercises;

pub fn main_roman_numeral_comparison() {
    print!("[EASY] roman_numeral_comparison\t");
    assert_eq!(exercises::roman_numeral_comparison::numcompare("XVI", "VIII"), false);
    assert_eq!(exercises::roman_numeral_comparison::numcompare("V", "IIIII"), false);
    assert_eq!(exercises::roman_numeral_comparison::numcompare("CCLXV", "MMDII"), true);
    println!("OK");
}
