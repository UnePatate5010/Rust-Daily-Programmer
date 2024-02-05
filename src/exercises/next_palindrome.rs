// https://www.reddit.com/r/dailyprogrammer/comments/n3var6/20210503_challenge_388_intermediate_next/

fn palindrome(s: String) -> bool {
    let v: Vec<char> = s.chars().collect();
    for i in 0..(v.len() / 2) {
        if v[i] != v[v.len() - i - 1] {
            return false;
        }
    }
    return true;
}

pub fn next_palindrome(x: u32) -> u32 {
    let mut n_x = x + 1;
    while !palindrome(n_x.to_string()) {
        n_x += 1;
    }
    return n_x;
}
