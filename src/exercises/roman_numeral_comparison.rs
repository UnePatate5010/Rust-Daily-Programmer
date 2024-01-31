// https://www.reddit.com/r/dailyprogrammer/comments/oe9qnb/20210705_challenge_397_easy_roman_numeral/

fn numsum(n1 : &str) -> i32 {
    let mut sum : i32 = 0;
    for i in n1.chars() {
        match i {
            'M' => sum += 1000,
            'D' => sum += 500,
            'C' => sum += 100,
            'L' => sum += 50,
            'X' => sum += 10,
            'V' => sum += 5,
            'I' => sum += 1,
            _ => continue,
        }
    }
    return sum;
}


pub fn numcompare(n1 : &str, n2 : &str) -> bool {
    numsum(n1) < numsum(n2)

}

