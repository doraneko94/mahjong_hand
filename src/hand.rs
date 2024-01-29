use crate::count::Count;
use crate::index::{index2pai, pai2name, Name};
use crate::naki::Naki;

pub struct Hand {
    pub stand: Vec<u8>,
    pub naki: Naki,
}

#[derive(Clone)]
pub struct HandCount {
    count: Count,
    naki: Naki,
}

impl Hand {
    pub fn new(haipai: &[u8; 13]) -> Self {
        let naki = Naki::new();
        let mut stand = haipai.to_vec();
        stand.sort();
        Self { stand, naki }
    }

    pub fn from_string(s: &str) -> Self {
        let mut stand = Vec::new();
        let naki = Naki::new();
        let mut mode = 0;
        for c in s.chars() {
            match c.to_digit(10) {
                Some(p) => {
                    let mut n = mode * 36 + (p - 1) as u8 * 4;
                    while stand.contains(&n) { n += 1;}
                    stand.push(n);
                }
                None => {
                    if c == 'm' { mode = 0; }
                    if c == 'p' { mode = 1; }
                    if c == 's' { mode = 2; }
                    if c == 'z' { mode = 3; }
                }
            }
        }
        Self { stand, naki }
    }

    pub fn tsumo(&mut self, pai: u8) {
        self.stand.push(pai);
        self.stand.sort();
    }

    pub fn dahai(&mut self, position: usize) -> u8 {
        self.stand.remove(position)
    }

    pub fn kan(&mut self, position: usize) -> bool {
        let n = self.stand.len();
        let pai = index2pai(self.stand[position]);
        let mut stand_new = self.stand.clone();
        stand_new.retain(|&si| index2pai(si) != pai);
        if n - 4 == stand_new.len() {
            self.stand = stand_new;
            self.naki.kan.push(pai2name(pai));
            true
        } else { false } 
    }

    pub fn to_count(&self) -> HandCount {
        HandCount::from_hand(&self)
    }

    pub fn is_agari(&self) -> bool {
        self.to_count().is_agari()
    }

    pub fn is_tenpai(&self) -> bool {
        self.to_count().is_tenpai()
    }
}

impl HandCount {
    pub fn from_hand(hand: &Hand) -> Self {
        let count = Count::from_indices(&hand.stand);
        let naki = hand.naki.clone();
        Self { count, naki }
    }

    pub fn is_agari(&self) -> bool {
        self.count.is_agari()
    }

    pub fn is_tenpai(&self) -> bool {
        for hc in self.virtual_tsumo().iter() {
            if hc.is_agari() { return true; }
        }
        false
    }

    pub fn virtual_tsumo(&self) -> Vec<HandCount> {
        let mut v = Vec::new();
        for pai in 0..33 {
            let mut hc = self.clone();
            let name = pai2name(pai);
            if !self.is_full(name) {
                hc.add_name(name);
                v.push(hc);
            }
        }
        v
    }

    pub fn is_full(&self, name: Name) -> bool {
        let in_count = if name.0  == 0 { self.count.manzu[name.1] == 4 }
            else if name.0 == 1 { self.count.pinzu[name.1] == 4 }
            else if name.0 == 2 { self.count.souzu[name.1] == 4 }
            else { self.count.zupai[name.1] == 4 };
        in_count || self.naki.kan.contains(&name)
    }

    pub fn add_name(&mut self, name: Name) {
        if name.0 == 0 { self.count.manzu[name.1] += 1; }
        if name.0 == 1 { self.count.pinzu[name.1] += 1; }
        if name.0 == 2 { self.count.souzu[name.1] += 1; }
        if name.0 == 3 { self.count.zupai[name.1] += 1; }
    }

    pub fn tanyao(&self) -> bool {
        self.count.manzu[0] == 0 && self.count.manzu[8] == 0
        && self.count.pinzu[0] == 0 && self.count.pinzu[8] == 0
        && self.count.souzu[0] == 0 && self.count.souzu[8] == 0
        && self.count.zupai.iter().map(|&zi| zi).sum::<u8>() == 0
    }

    pub fn honroutou(&self) -> bool {
        let mut flg = self.count.manzu[1..8].iter().map(|&i| i).sum::<u8>() == 0
        && self.count.pinzu[1..8].iter().map(|&i| i).sum::<u8>() == 0
        && self.count.souzu[1..8].iter().map(|&i| i).sum::<u8>() == 0;
        for &ki in self.naki.kan.iter() {
            if ki.0 != 3 && ki.1 != 0 && ki.1 != 8 { flg = false; }
        }
        flg
    }

    pub fn kantsu3(&self) -> bool { self.naki.kan.len() == 3 }
}