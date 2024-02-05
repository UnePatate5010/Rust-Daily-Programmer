use crate::exercises::roman_numeral_comparison;

pub fn main_roman_numeral_comparison() {
    print!("#397 [EASY] roman_numeral_comparison\t");
    assert_eq!(roman_numeral_comparison::numcompare("XVI", "VIII"), false);
    assert_eq!(roman_numeral_comparison::numcompare("V", "IIIII"), false);
    assert_eq!(roman_numeral_comparison::numcompare("CCLXV", "MMDII"), true);
    println!("OK");
}
