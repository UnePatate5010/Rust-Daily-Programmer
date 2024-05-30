use crate::exercises::iec_rule;

pub fn main_iec_rule() {
    print!("#363 [EASY] I before E except after C\t");
    assert_eq!(iec_rule::iec_rule("a"), true);
    assert_eq!(iec_rule::iec_rule("zombie"), true);
    assert_eq!(iec_rule::iec_rule("transceiver"), true);
    assert_eq!(iec_rule::iec_rule("veil"), false);
    assert_eq!(iec_rule::iec_rule("icier"), false);
    println!("OK");
}