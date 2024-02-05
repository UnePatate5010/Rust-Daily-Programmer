// https://www.reddit.com/r/dailyprogrammer/comments/3opin7/20151014_challenge_236_intermediate_fibonacciish/

fn fibo(x: u32, start: u32) -> u32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return start;
    } else {
        return fibo(x - 1, start) + fibo(x - 2, start);
    }
}

pub fn fibo_sequence(target: u32) -> u32 {
    let mut start: u32 = 1;
    loop {
        let mut i: u32 = 1;
        let mut f = fibo(i, start);
        while f < target {
            i += 1;
            f = fibo(i, start);
        }
        if f == target {
            return start;
        }
        start += 1;
    }
}

pub fn fibo_sequence_2(target: u32) -> u32 {
    let mut div: u32 = 1;
    let mut i: u32 = 1;
    let mut f = fibo(i, 1);
    while f <= target {
        if target % f == 0 {
            div = f;
        }
        i += 1;
        f = fibo(i, 1);
    }
    return target / div;
}
