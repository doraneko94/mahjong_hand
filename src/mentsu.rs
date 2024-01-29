use crate::count::Count;

#[derive(Clone, PartialEq)]
pub struct Mentsu {
    pub mode: u8,
    pub num: [usize; 3],
}

impl Mentsu {
    pub fn new(mode: u8, num: [usize; 3]) -> Self {
        Self { mode, num }
    }

    pub fn is_anko(&self) -> bool { self.num[0] == self.num[1] && self.num[1] == self.num[2] }
    pub fn is_yaochu(&self) -> bool {
        if self.mode == 3 { return true; }
        for i in 0..3 {
            if self.num[i] == 0 || self.num[i] == 8 { return true; }
        }
        false
    }
    pub fn is_jun(&self) -> bool { self.is_yaochu() && self.mode != 3 }
}

fn _decomp(c: &mut [u8; 9], mode: u8, n: usize) -> Vec<Vec<Mentsu>> {
    if n >= 9 { return vec![vec![]]; }
    if c[n] == 0 { return _decomp(c, mode, n+1) }

    let mut shuntsu = Vec::new();
    if n < 7 && c[n] > 0 && c[n + 1] > 0 && c[n + 2] > 0 {
        c[n] -= 1; c[n + 1] -= 1; c[n + 2] -= 1;
        shuntsu = _decomp(c, mode, n);
        for s in shuntsu.iter_mut() {
            s.push(Mentsu::new(mode, [n, n+1, n+2]));
        }
        c[n] += 1; c[n + 1] += 1; c[n + 2] += 1;
    }
    
    let mut koutsu = Vec::new();
    if c[n] >= 3 {
        c[n] -= 3;
        koutsu = _decomp(c, mode, n);
        c[n] += 3;
        for t in koutsu.iter_mut() {
            t.push(Mentsu::new(mode, [n; 3]));
        }
    }
    for t in koutsu.iter() {
        shuntsu.push(t.clone());
    }
    shuntsu
}

pub fn decompose(c: &Count) -> Vec<Vec<Mentsu>> {
    let mut v_all = vec![vec![]];
    for (i, ci) in [c.manzu, c.pinzu, c.souzu].iter().enumerate() {
        let mut c = ci.clone();
        let mut v_new = Vec::new();
        let v_sub = _decomp(&mut c, i as u8, 0);

        for va in v_all.iter() {
            for vs in v_sub.iter() {
                let mut v = va.clone();
                for m in vs.iter() {
                    v.push(m.clone());
                }
                v_new.push(v);
            }
        }
        v_all = v_new;
    }
    
    v_all
}