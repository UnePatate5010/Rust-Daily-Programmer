// https://www.reddit.com/r/dailyprogrammer/comments/55nior/20161003_challenge_286_easy_reverse_factorial/

pub fn reverse_factorial(x: u32) -> bool {
    let mut i: u32 = 2;
    let mut n_x: u32 = x;
    while n_x % i == 0 {
        n_x = n_x / i;
        i += 1;
    }
    if n_x == 1 {
        return true;
    }
    else {
        return false;
    }
}
