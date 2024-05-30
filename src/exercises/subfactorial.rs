// https://www.reddit.com/r/dailyprogrammer/comments/9cvo0f/20180904_challenge_367_easy_subfactorials_another/

pub fn subfactorial(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 0,
        _ => (n - 1) * (subfactorial(n - 1) + subfactorial(n - 2)),
    }
}
