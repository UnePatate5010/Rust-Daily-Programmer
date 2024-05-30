// https://www.reddit.com/r/dailyprogrammer/comments/8q96da/20180611_challenge_363_easy_i_before_e_except/

pub fn iec_rule(word: &str) -> bool {
    let w: Vec<char> = word.chars().collect();
    for i in 0..w.len() - 1 {
        if w[i] == 'e' && w[i + 1] == 'i' && (i == 0 || !(w[i - 1] == 'c')) {
            return false;
        }
        if w[i] == 'i' && w[i + 1] == 'e' && i > 0 && w[i - 1] == 'c' {
            return false;
        }
    }
    return true;
}
