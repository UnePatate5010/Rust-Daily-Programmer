// https://www.reddit.com/r/dailyprogrammer/comments/b0nuoh/20190313_challenge_376_intermediate_the_revised/

pub fn julian_calendar(y1: u32, y2:u32) -> u32{
    let mut count: u32 = 0;
    for i in y1..y2 {
        if i % 4 == 0 && (!(i % 100 == 0) || (i % 900 == 200) || (i % 900 == 600)) {
            count += 1
        }
    }
    return count;
}

// A more efficient way would be to only check multiples of 4 instead of looping through every year between y1 and y2