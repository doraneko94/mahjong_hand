use crate::count::Count;

pub fn is_mentsu(c: &[u8]) -> bool {
    let (mut a, mut b) = (c[0], c[1]);

    for i in 0..7 {
        let r = a % 3;
        if b >= r && c[i + 2] >= r {
            a = b - r;
            b = c[i + 2] - r;
        } else {
            return false;
        }
    }
    if a % 3 == 0 && b % 3 == 0 { true } else { false }
}

pub fn is_anko(c: &[u8]) -> bool {
    for &ci in c.iter() {
        if ci % 3 != 0 { return false; }
    }
    true
}

pub fn is_agari(count: &Count) -> bool {
    let cands = count.find_head();
    for (_, c) in cands {
        if is_mentsu(&c.manzu) && is_mentsu(&c.pinzu)
        && is_mentsu(&c.souzu) && is_anko(&c.zupai) {
            return true;
        }
    }
    false
}