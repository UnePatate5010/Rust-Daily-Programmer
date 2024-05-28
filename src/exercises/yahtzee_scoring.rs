// https://www.reddit.com/r/dailyprogrammer/comments/onfehl/20210719_challenge_399_easy_letter_value_sum/

pub fn yahtzee_scoring(arr: &Vec<i32>) -> i32 {
    let mut bsf: i32 = 0;
    for i in 1..7 {
        let mut count = 0;
        for num in arr {
            if *num == i {
                count += 1;
            }
        }
        let score = count * i;
        if score > bsf {
            bsf = score;
        }
    }
    return bsf;
}


// Bonus version
pub fn yahtzee_scoring2(arr: &Vec<i32>) -> i32 {
    let mut bsf: i32 = 0;

    for i in arr {
        let mut count = 0;
        for j in arr {
            if *i == *j {
                count += 1;
            }
        }
        let score = count * *i;
        if score > bsf {
            bsf = score;
        }
    }

    return bsf;
}
