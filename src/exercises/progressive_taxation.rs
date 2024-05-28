// https://www.reddit.com/r/dailyprogrammer/comments/cdieag/20190715_challenge_379_easy_progressive_taxation/

pub fn tax(mut income: u32) -> u32{
    let mut t: f32 = 0.0;
    if income > 100000 {
        t += (income - 100000) as f32 * 0.4;
        income = 100000;  
    }
    if income > 30000 {
        t += (income - 30000) as f32 * 0.25;
        income = 30000;
    }
    if income > 10000 {
        t += (income - 10000) as f32 * 0.1;
    }
    return t as u32;
}