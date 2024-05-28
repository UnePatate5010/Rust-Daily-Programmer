// https://www.reddit.com/r/dailyprogrammer/comments/onfehl/20210719_challenge_399_easy_letter_value_sum/


fn letter_value(letter: char) -> u32 {
    return (letter as u32) - ('a' as u32) + 1;
}

pub fn letter_sum_value(word: &str) -> u32 {
    let mut sum = 0;
    for i in word.chars() {
        sum += letter_value(i);
    }
    return sum;
}