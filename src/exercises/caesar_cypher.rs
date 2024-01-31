// https://www.reddit.com/r/dailyprogrammer/comments/myx3wn/20210426_challenge_387_easy_caesar_cipher/

fn shift(letter: char, shifting: u8) -> char {
    if letter.is_whitespace() {
        return letter;
    }
    let real_shifting: u8 = shifting % 26;
    let letter_number = letter as u8;
    let mut new_number = letter_number + real_shifting;
    if new_number > 'z' as u8 {
        new_number -= 26;
    }
    let new_letter = new_number as char;
    return new_letter;
}

pub fn caesar_cypher(sentence: &str, shifting: i32) -> String {
    let new_shifting: u8 = ((shifting % 26) + 26) as u8;
    let mut code_sentence = String::new();
    for i in sentence.chars() {
        code_sentence.push(shift(i, new_shifting));
    }
    return code_sentence;
}

pub fn caesar_decode(sentence: &str, shifting: i32) -> String {
    return caesar_cypher(sentence, shifting);
}


