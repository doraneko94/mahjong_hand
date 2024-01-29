use crate::index::Name;
use crate::mentsu::Mentsu;
use crate::naki::Naki;

const ALLGREEN: [Name; 6] = [(2, 1), (2, 2), (2, 3), (2, 5), (2, 7), (3, 5)];

pub struct Calc {
    stand: Vec<Mentsu>,
    last: Option<Mentsu>,
    naki: Naki,
    head: Name,
    tsumo: Name,
    han_base: usize,
}

impl Calc {
    pub fn new(
        stand: Vec<Mentsu>,
        last: Option<Mentsu>,
        naki: Naki,
        head: Name,
        tsumo: Name,
        han_base: usize,
    ) -> Result<Self, ()> {
        if stand.len() + naki.kan.len() != 4 { return Err(()); }
        Ok(Calc { stand, last, naki, head, tsumo, han_base })
    }

    fn _contain(&self, name: Name) -> bool {
        self.stand.contains(&Mentsu::new(name.0, [name.1; 3])) || self.naki.kan.contains(&name)
    }

    fn _contain_head(&self, name: Name) -> bool {
        self._contain(name) || self.head == name
    }

    pub fn yakuman(&self) -> usize {
        let mut ans = 0;

        // all green
        let mut flg = true;
        for m in self.stand.iter() {
            for i in 0..2 { if !ALLGREEN.contains(&(m.mode, m.num[i])) { flg = false; } }
        }
        for n in self.naki.kan.iter() {
            if !ALLGREEN.contains(n) { flg = false; }
        }
        if flg { ans += 1; }

        // daisangen
        if self._contain((3, 4)) && self._contain((3, 5)) && self._contain((3, 6)) {
            ans += 1;
        }
        
        // shousushi
        if self._contain_head((3, 0)) && self._contain_head((3, 1))
        && self._contain_head((3, 2)) && self._contain_head((3, 3)) {
            ans += 1;
            // daisushi
            if self.head != (3, 0) && self.head != (3, 1) && self.head != (3, 2) && self.head != (3, 3) {
                ans += 1;
            }
        }

        // zuiso
        flg = true;
        for m in self.stand.iter() { if m.mode != 3 { flg = false; } }
        for k in self.naki.kan.iter() { if k.0 != 3 { flg = false; } }
        if self.head.0 != 3 { flg = false; }
        if flg { ans += 1; }

        // 4anko
        if self.stand.iter().fold(true, |b, m| b && m.is_anko()) {
            ans += 1;
            if self.last.is_none() { ans += 1; }
        }

        // chinroto
        flg = true;
        for m in self.stand.iter() {
            if m.mode == 3 || !m.is_anko() || (m.num[0] != 0 && m.num[0] != 8) { flg = false; }
        }
        
        ans
    }
}