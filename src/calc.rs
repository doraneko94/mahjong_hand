use crate::index::{Name, Yaku};
use crate::mentsu::Mentsu;
use crate::naki::Naki;
use crate::score::Score;

pub struct Calc {
    stand: Vec<Mentsu>,
    last: Option<Mentsu>,
    naki: Naki,
    head: Name,
    tsumo: Name,
    han_base: usize,
    ba: usize,
    cha: usize,
}

impl Calc {
    pub fn new(
        stand: &[Mentsu],
        last: Option<Mentsu>,
        naki: Naki,
        head: Name,
        tsumo: Name,
        han_base: usize,
        ba: usize,
        cha: usize,
    ) -> Result<Self, ()> {
        if stand.len() + naki.kan.len() != 4 { return Err(()); }
        Ok(Calc { stand: stand.to_vec(), last, naki, head, tsumo, han_base, ba, cha })
    }

    fn _contain(&self, name: Name) -> bool {
        self.stand.contains(&Mentsu::new(name.0, [name.1; 3])) || self.naki.kan.contains(&name)
    }

    fn _contain_head(&self, name: Name) -> bool {
        self._contain(name) || self.head == name
    }

    pub fn score(&self) -> Score {
        let mut fu = self.fu();
        let mut han = self.han();
        if fu == 20 { han += 1; } else { fu += 2; } // tsumo & pinfu
        Score::new(fu, han, self.cha == 0)
    }

    pub fn fu(&self) -> usize { // no tsumo
        let mut f = 20;

        // head
        if self.head.0 == 3 {
            if self.head.1 >= 4 || self.head.1 == self.ba || self.head.1 == self.cha { f += 2; }
        }
        // anko
        for m in self.stand.iter() { 
            if m.is_anko() {
                if m.is_yaochu() { f += 8; } else { f += 4; }
            }
        }
        // kantsu
        for k in self.naki.kan.iter() {
            if k.is_yaochu() { f += 32; } else { f += 16; }
        }
        // machi
        match &self.last {
            None => { f += 2; }
            Some(m) => {
                if !m.is_anko() {
                    if m.num[0] == self.tsumo.1 && m.num[2] == 8 { f += 2; }
                    if m.num[1] == self.tsumo.1 { f += 2; }
                    if m.num[2] == self.tsumo.1 && m.num[0] == 0 { f += 2; }
                }
            }
        }
        f
    }

    pub fn han(&self) -> usize { // no pinfu
        // han_base = dora + aka + tsumo + reach + double + ippatsu + tanyao + haitei + rinshan
        let mut h = self.han_base;
        let n = self.stand.len();
        // 1,2peiko
        let mut opt = None;
        for i in 0..n {
            for j in i+1..n {
                if !self.stand[i].is_anko() && self.stand[i] == self.stand[j] {
                    match &opt {
                        None => {
                            opt = Some(self.stand[i].clone());
                            h += 1;
                        }
                        Some(m) => {
                            if self.stand[i] != *m { h += 2; }
                        }
                    }
                }
            }
        }
        // yakuhai
        for m in self.stand.iter() {
            if m.mode == 3 {
                if m.num[0] >= 4 { h += 1; }
                if m.num[0] == self.ba { h += 1; }
                if m.num[0] == self.cha { h += 1; }
            }
        }
        for k in self.naki.kan.iter() {
            if k.0 == 3 {
                if k.1 >= 4 { h += 1; }
                if k.1 == self.ba { h += 1; }
                if k.1 == self.cha { h += 1; }
            }
        }
        // toitoi
        if self.stand.iter().fold(true, |b, m| b && m.is_anko()) { h += 2; }
        // 3anko
        if self.stand.iter().map(|m| if m.is_anko() { 1 } else { 0 }).sum::<usize>() + self.naki.kan.len() == 3 { h += 2; }
        // 3shoku-douko
        let mut koutsu = Vec::new();
        for m in self.stand.iter() { if m.is_anko() { koutsu.push(m.num[0]); } }
        for k in self.naki.kan.iter() { koutsu.push(k.1); }
        let n_kou = koutsu.len();
        for i in 0..n_kou {
            let mut c = 1;
            for j in i+1..n_kou {
                if koutsu[i] == koutsu[j] { c += 1; }
            }
            if c == 3 {
                h += 2;
                break;
            }
        }
        // 3shoku-doujun
        for i in 0..n {
            if self.stand[i].is_anko() { continue; }
            let mut v = [false; 3];
            v[self.stand[i].mode as usize] = true;
            for j in i+1..n {
                if !self.stand[j].is_anko() && self.stand[i].num[0] == self.stand[j].num[0] {
                    v[self.stand[j].mode as usize] = true;
                }
            }
            if v[0] && v[1] && v[2] {
                h += 2;
                break;
            }
        }
        // hourontou & chanta
        let mut flg = self.naki.kan.iter().fold(true, |b, k| b && k.is_yaochu());
        flg = self.stand.iter().fold(flg, |b, m| b && m.is_yaochu());
        if flg {
            h += 2;
            if self.naki.kan.iter().fold(true, |b, m| b && m.is_jun()) {
                h += 1;
            }
        }
        // ikki
        for i in 0..2 {
            let mut v = [false; 3];
            for m in self.stand.iter() {
                if !m.is_anko() && m.mode == i && m.num[0] % 3 == 1 {
                    v[m.num[0] / 3] = true;
                }
            }
            if v[0] && v[1] && v[2] {
                h += 2;
                break;
            }
        }
        // shosangen
        if self._contain_head((3, 4)) && self._contain_head((3, 5)) && self._contain_head((3, 6)) {
            h += 2;
        }
        // 3kantsu
        if self.naki.kan.len() == 3 { h += 2; }
        // isshoku
        let mut v = [0; 4];
        for m in self.stand.iter() { v[m.mode as usize] = 1; }
        for k in self.naki.kan.iter() { v[k.0 as usize] = 1; }
        if v[..3].iter().sum::<usize>() == 1 {
            h += 3;
            if v[3] == 0 { h += 3; }
        }

        h
    }
    
}