use mahjong_hand::count::Count;
// use mahjong_hand::hand::Hand;
use mahjong_hand::mentsu::decompose;
use mahjong_hand::score::Score;

fn main() {
    let score = Score::new(6, 33, true);
    println!("{}, {:?}", score.children, score.parent);

    let c = Count {
        manzu: [3,3,3,0,0,0,0,0,0],
        pinzu: [1,1,1,0,0,0,0,0,0],
        souzu: [0; 9], zupai: [0; 7]
    };
    let ans = decompose(&c);
    for a in ans.iter() {
        for m in a.iter() {
            print!("({} {:?}) ", m.mode, m.num);
        }
        println!("");
    }
    println!("{}", ans.len());
}
