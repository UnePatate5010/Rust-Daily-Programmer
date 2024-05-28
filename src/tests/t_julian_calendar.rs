use crate::exercises::julian_calendar;

pub fn main_julian_calendar() {
    print!("#376 [INTERMEDIATE] Revised julian calendar\t");
    assert_eq!(julian_calendar::julian_calendar(2016, 2017), 1);
    assert_eq!(julian_calendar::julian_calendar(2000, 2001), 1);
    assert_eq!(julian_calendar::julian_calendar(2800, 2801), 0);
    assert_eq!(julian_calendar::julian_calendar(1234, 5678), 1077);
    println!("OK");
}