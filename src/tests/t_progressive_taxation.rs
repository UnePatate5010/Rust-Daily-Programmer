use crate::exercises::progressive_taxation;


pub fn main_progressive_taxationt() {
    print!("#379 [EASY] Progressive taxation\t");
    assert_eq!(progressive_taxation::tax(0), 0);
    assert_eq!(progressive_taxation::tax(10010), 1);
    assert_eq!(progressive_taxation::tax(56789), 8697);
    assert_eq!(progressive_taxation::tax(1234567), 473326);
    println!("OK");
}