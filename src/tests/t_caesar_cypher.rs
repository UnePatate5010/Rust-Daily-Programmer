use crate::exercises::caesar_cypher;

pub fn main_caesar_cypher() {
    print!("[EASY] caesar_cypher\t");
    assert_eq!(caesar_cypher::caesar_cypher("hello bitches", 8), "pmttw jqbkpma");
    assert_eq!(caesar_cypher::caesar_decode("ifmmp cjudift", -1), "hello bitches");
    println!("OK");
}
