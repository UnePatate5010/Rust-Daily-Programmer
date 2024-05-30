// https://www.reddit.com/r/dailyprogrammer/comments/99yl83/20180824_challenge_366_hard_incomplete_word/

fn spacing(word1: &str, word2: &str) -> i32 {
    let mut count: i32 = 0;
    let mut nb_char: i32 = 0;
    let mut w2: Vec<char> = word2.chars().collect();
    for i in word1.chars() {
        nb_char += 1;
        for j in 0..w2.len() {
            if i == w2[j] {
                count += 1;
                w2.remove(j);
                break;
            }
        }
    }

    return nb_char - count - 1;
}

pub fn total_spacing(words: &Vec<&str>) -> i32 {
    if words.len() <= 1 {
        return 0;
    }
    let mut sum = 0;
    for i in 0..words.len() - 1 {
        sum += spacing(words[i], words[i + 1]);
    }
    return sum;
}

fn power_set<T: Clone>(vec: &Vec<T>) -> Vec<Vec<T>> {
    let n = vec.len();
    let mut subsets = Vec::new();

    // There are 2^n subsets
    for i in 0..1 << n {
        let mut subset = Vec::new();
        for j in 0..n {
            // Check if the jth element is included in the ith subset
            if (i & (1 << j)) != 0 {
                subset.push(vec[j].clone());
            }
        }
        subsets.push(subset);
    }

    subsets
}

pub fn word_ladder<'a>(words: &'a Vec<&'a str>, max_spacing: u32) -> (Vec<&'a str>, i32) {
    let mut bsf_vec = vec![""];
    let mut bsf_spacing = 0;
    for subset in power_set(&words) {
        let current_spacing = total_spacing(&subset);
        if current_spacing < (max_spacing as i32) && subset.len() > bsf_vec.len() {
            bsf_vec = subset;
            bsf_spacing = current_spacing;
        }
    }
    return (bsf_vec, bsf_spacing);
}
