// https://www.reddit.com/r/dailyprogrammer/comments/onfehl/20210719_challenge_399_easy_letter_value_sum/

pub fn pancake_sort(arr : &Vec<i32>, n: usize) -> Vec<i32> {
    let mut a = Vec::with_capacity(arr.len());
    for i in 0..n {
        a.push(arr[n-1 - i]);
    }
    for i in n..arr.len() {
        a.push(arr[i]);
    }
    return a
}   