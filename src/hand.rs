use crate::calc::Calc;
use crate::count::Count;
use crate::index::{index2pai, pai2name, Name};
use crate::mentsu::decompose;
use crate::naki::Naki;
use crate::score::Score;

const ALLGREEN: [Name; 6] = [(2, 1), (2, 2), (2, 3), (2, 5), (2, 7), (3, 5)];

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

    pub fn score(
        &self,
        tenhou: bool,
        reach: bool,
        double: bool,
        ippatsu: bool,
        haitei: bool,
        rinshan: bool,
        dora: &[Name],
        tsumo: Name,
        ba: usize,
        cha: usize
    ) -> Option<Score> {
        let hc = self.to_count();
        if !hc.is_agari() { return None; }

        let bai = hc.yakuman(tenhou, tsumo);
        if bai > 0 {
            return Some(
                Score {
                    parent: if cha == 0 { None } else { Some(16000 * bai) },
                    children: if cha == 0 { 16000 * bai } else { 8000 * bai },
                    fu : 0,
                    han: bai,
                }
            )
        }
        
        let mut han_base: usize = 1 // tsumo
        + if reach { 1 } else { 0 }
        + if double { 1 } else { 0 }
        + if ippatsu { 1 } else { 0 }
        + if haitei { 1 } else { 0 }
        + if rinshan { 1 } else { 0 };

        let mut count = vec![
            hc.count.manzu.to_vec(),
            hc.count.pinzu.to_vec(),
            hc.count.souzu.to_vec(),
            hc.count.zupai.to_vec(),
        ];
        for k in hc.naki.kan.iter() {
            count[k.0 as usize][k.1] += 4;
        }
        // dora
        for d in dora.iter() {
            han_base += count[d.0 as usize][d.1] as usize;
        }
        // aka
        if self.stand.contains(&16) { han_base += 1; }
        if self.stand.contains(&(36+16)) { han_base += 1; }
        if self.stand.contains(&(36*2+16)) { han_base += 1; }
        // isshoku
        let s = (0..4).map(|i| if count[i].iter().sum::<u8>() > 0 { 1 } else { 0 }).collect::<Vec<u8>>();
        let js = count[0][0] + count[0][8] + count[1][0] + count[1][8] + count[2][0] + count[2][8] > 0;
        let ts = (0..3).map(|i| count[i][1..8].iter().sum::<u8>()).sum::<u8>() > 0;
        if s[0] + s[1] + s[2] == 1 {
            han_base += 3; // honitsu
            if s[3] == 0 { han_base += 3; } // chinitsu
        }
        if s[3] == 0 && !js { han_base += 1; } // tanyao
        if !ts { han_base += 2; } // honroutou

        let mut scores = Vec::new();
        if hc.count.is_7toitsu() {
            scores.push(Score::new(25, han_base + 2, cha == 0));
        }
        let heads = hc.count.find_head();
        for (hi, c) in heads.iter() {
            let vms = decompose(c);
            for vm in vms.iter() {
                if *hi == tsumo {
                    match Calc::new(vm, None, hc.naki.clone(), hi.clone(), tsumo, han_base, ba, cha) {
                        Ok(ca) => { scores.push(ca.score()) }
                        Err(_) => {}
                    }
                }
                for m in vm.iter() {
                    if m.mode == tsumo.0 && m.num.contains(&tsumo.1) {
                        match Calc::new(vm, Some(m.clone()), hc.naki.clone(), hi.clone(), tsumo, han_base, ba, cha) {
                            Ok(ca) => { scores.push(ca.score()) }
                            Err(_) => {}
                        }
                    }
                }
            }
        }
        let mut ans = scores[0].clone();
        for i in 1..scores.len() {
            if ans.children < scores[i].children {
                ans = scores[i].clone();
            }
        }

        Some(ans)
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
        && self.count.zupai.iter().sum::<u8>() == 0
    }

    pub fn yakuman(&self, tenhou: bool, tsumo: Name) -> usize {
        let mut ans = if tenhou { 1 } else { 0 };

        // 4kantsu
        if self.naki.kan.len() == 4 { ans += 1; }

        let mut count = vec![
            self.count.manzu.to_vec(),
            self.count.pinzu.to_vec(),
            self.count.souzu.to_vec(),
            self.count.zupai.to_vec(),
        ];
        // kokushi
        let mut flg = true;
        let mut last = (0, 0);
        for i in 0..3 {
            for j in [0, 8] {
                if count[i][j] == 1 {} else if count[i][j] == 2 { last = (i as u8, j); } else { flg = false; }
            }
        }
        for j in 0..7 {
            if count[3][j] == 1 {} else if count[3][j] == 2 { last = (3, j); } else { flg = false; }
        }
        if flg {
            ans += 1;
            if last == tsumo { ans += 1; }
        }
        // 9ren
        let v:[u8; 4] = [
            if count[0].iter().sum::<u8>() > 0 { 1 } else { 0 },
            if count[1].iter().sum::<u8>() > 0 { 1 } else { 0 },
            if count[2].iter().sum::<u8>() > 0 { 1 } else { 0 },
            if count[3].iter().sum::<u8>() > 0 { 1 } else { 0 }
        ];
        if v.iter().sum::<u8>() == 1 && v[3] == 0 {
            let c = if v[0] == 1 {
                count[0].clone()
            } else if v[1] == 1 {
                count[1].clone()
            } else {
                count[2].clone()
            };
            flg = true;
            let mut last = 0;
            for i in [0, 8] {
                if c[i] == 3 {} else if c[i] == 4 {
                    last = i;
                } else { flg = false; }
            }
            for i in 1..8 {
                if c[i] == 1 {} else if c[i] == 2 {
                    last = i;
                } else { flg = false; }
            }
            if flg {
                ans += 1;
                if last == tsumo.1 { ans += 1; }
            }
        }
        
        for k in self.naki.kan.iter() {
            count[k.0 as usize][k.1] += 3;
        }
        
        // all green
        flg = true;
        for i in 0..4 {
            for j in 0..count[i].len() {
                if count[i][j] > 0 {
                    if !ALLGREEN.contains(&(i as u8, j)) { flg = false; }
                }
            }
        }
        if flg { ans += 1; }
        // daisangen
        if count[3][4] == 3 && count[3][5] == 3 && count[3][6] == 3 { ans += 1; }
        // shousushi daisushi
        let s = count[3][0..4].iter().sum::<u8>();
        if s >= 11 { ans += 1; }
        if s == 12 { ans += 1; }
        // zuiso
        if (0..3).map(|i| count[i].iter().sum::<u8>()).sum::<u8>() == 0 { ans += 1; }
        // 4anko
        flg = true;
        last = (0, 0);
        for i in 0..4 {
            for j in 0..count[i].len() {
                if count[i][j] == 3 {} else if count[i][j] == 2 {
                    last = (i as u8, j);
                } else { flg = false; }
            }
        }
        if flg {
            ans += 1;
            if last == tsumo { ans += 1; }
        }
        // chinroutou
        if (0..3).map(|i| count[i][1..8].iter().sum::<u8>()).sum::<u8>() + count[3].iter().sum::<u8>() == 0 { ans += 1; }
        
        ans
    }
}