// https://www.reddit.com/r/dailyprogrammer/comments/onfehl/20210719_challenge_399_easy_letter_value_sum/

pub fn sequence(n: u32) -> String {
    // assert_eq!(n <= 26, true);
    let mut word = "a".to_string();
    for i in 1..n {
        let letter: char = (('a' as u32) + i) as u8 as char;
        word = format!("{}{}", word, letter) + &word;
    }
    return word;
}