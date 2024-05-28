use crate::exercises::letter_sum_value;

pub fn main_letter_sum_value() {
    print!("#399 [Easy] Letter value sum\t");
    assert_eq!(letter_sum_value::letter_sum_value(""), 0);
    assert_eq!(letter_sum_value::letter_sum_value("a"), 1);
    assert_eq!(letter_sum_value::letter_sum_value("z"), 26);
    assert_eq!(letter_sum_value::letter_sum_value("microspectrophotometries"), 317);    
    println!("OK");
}