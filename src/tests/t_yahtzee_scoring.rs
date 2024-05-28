use crate::exercises::yahtzee_scoring;

pub fn main_yahtzee_scoring() {
    print!("#381 [EASY] Yahtzee upper section scoring\t");
    assert_eq!(yahtzee_scoring::yahtzee_scoring(&vec![2, 3, 5, 5, 6]), 10);
    assert_eq!(yahtzee_scoring::yahtzee_scoring(&vec![1, 1, 1, 1, 3]), 4);
    assert_eq!(yahtzee_scoring::yahtzee_scoring(&vec![1, 1, 1, 3, 3]), 6);
    assert_eq!(yahtzee_scoring::yahtzee_scoring(&vec![6, 6, 6, 6, 6]), 30);
    assert_eq!(
        yahtzee_scoring::yahtzee_scoring2(
            &vec![
                1654,
                1654,
                50995,
                30864,
                1654,
                50995,
                22747,
                1654,
                1654,
                1654,
                1654,
                1654,
                30864,
                4868,
                1654,
                4868,
                1654,
                30864,
                4868,
                30864
            ]
        ),
        123456
    );
    println!("OK");
}
