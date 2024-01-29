use crate::agari::is_agari;
use crate::index::{index2name, Name};

#[derive(Clone)]
pub struct Count {
    pub manzu: [u8; 9],
    pub pinzu: [u8; 9],
    pub souzu: [u8; 9],
    pub zupai: [u8; 7],
}

fn _remove(v: &mut [u8], pos: usize, count: u8) -> Result<(), ()> {
    if v[pos] < count {
        Err(())
    } else {
        v[pos] -= count;
        Ok(())
    }
}

fn _find_head_part(count: &Count, v: &mut Vec<(Name, Count)>, u: &[u8], mode: u8) {
    for (i, &ui) in u.iter().enumerate() {
        if ui >= 2 {
            v.push(((mode, i), count.remove_head((mode, i)).unwrap()));
        }
    }
}
 
impl Count {
    pub fn empty() -> Self {
        Self { manzu: [0; 9], pinzu: [0; 9], souzu: [0; 9], zupai: [0; 7] }
    }

    pub fn from_indices(indices: &[u8]) -> Self {
        let mut c = Self::empty();
        for &index in indices.iter() {
            let name = index2name(index);
            if name.0 == 0 { c.manzu[name.1] += 1; }
            if name.0 == 1 { c.pinzu[name.1] += 1; }
            if name.0 == 2 { c.souzu[name.1] += 1; }
            if name.0 == 3 { c.zupai[name.1] += 1; }
        }
        c
    }

    pub fn remove_head(&self, name: Name) -> Result<Self, ()> {
        let mut manzu = self.manzu.clone();
        let mut pinzu = self.pinzu.clone();
        let mut souzu = self.souzu.clone();
        let mut zupai = self.zupai.clone();
        if name.0 == 0 { _remove(&mut manzu, name.1, 2)?; }
        if name.0 == 1 { _remove(&mut pinzu, name.1, 2)?; }
        if name.0 == 2 { _remove(&mut souzu, name.1, 2)?; }
        if name.0 == 3 { _remove(&mut zupai, name.1, 2)?; }
        Ok(Self { manzu, pinzu, souzu, zupai })
    }

    pub fn is_7toitsu(&self) -> bool {
        let mut count = 0;
        count += self.manzu.iter().map(|&i| if i == 2 { 1 } else { 0 }).sum::<u8>();
        count += self.pinzu.iter().map(|&i| if i == 2 { 1 } else { 0 }).sum::<u8>();
        count += self.souzu.iter().map(|&i| if i == 2 { 1 } else { 0 }).sum::<u8>();
        count += self.zupai.iter().map(|&i| if i == 2 { 1 } else { 0 }).sum::<u8>();
        count == 7
    }

    pub fn is_kokushi(&self) -> bool {
        for i in [0, 8] {
            if self.manzu[i] != 1 && self.manzu[i] != 2 { return false; }
            if self.pinzu[i] != 1 && self.pinzu[i] != 2 { return false; }
            if self.souzu[i] != 1 && self.souzu[i] != 2 { return false; }
        }
        for i in 0..7 {
            if self.zupai[i] != 1 && self.zupai[i] != 2 { return false; }
        }
        true
    }

    pub fn find_head(&self) -> Vec<(Name, Self)> {
        let mut v = Vec::new();
        _find_head_part(&self, &mut v, &self.manzu, 0);
        _find_head_part(&self, &mut v, &self.pinzu, 1);
        _find_head_part(&self, &mut v, &self.souzu, 2);
        _find_head_part(&self, &mut v, &self.zupai, 3);
        v
    }

    pub fn is_agari(&self) -> bool {
        self.is_7toitsu() || self.is_kokushi() || is_agari(&self)
    }
}