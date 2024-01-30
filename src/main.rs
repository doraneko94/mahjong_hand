use mahjong_hand::hand::Hand;

fn main() {
    let hand = Hand::from_string("m11112345678999");
    let score = hand.score(
        false, 
        true, 
        false, 
        false, 
        false, 
        false,
        &[(2, 3), (1, 8)],
        (0, 0),
        0, 1).unwrap();
    println!("{}-{}, {}-{:?}", score.fu, score.han, score.children, score.parent);
}
