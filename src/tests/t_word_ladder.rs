use crate::exercises::word_ladder::{self, word_ladder};

pub fn main_word_ladder() {
    print!("#366 [HARD] Incomplete word ladders\t");
    let words = vec![
        "abuzz",
        "carts",
        "curbs",
        "degas",
        "fruit",
        "ghost",
        "jupes",
        "sooth",
        "weirs",
        "zebra"
    ];
    assert_eq!(word_ladder::total_spacing(&words), 25);
    let words2 = vec!["daily",
    "doily",
    "golly",
    "guilt"];
    assert_eq!(word_ladder::total_spacing(&words2), 3);
    
    let (w, s) = word_ladder(&words, 10);
    let k = vec!["abuzz", "carts", "curbs", "degas", "ghost", "sooth"];
    assert!(s < 10);
    assert_eq!(w, k);

    println!("OK");
}
