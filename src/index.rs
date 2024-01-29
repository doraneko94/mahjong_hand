// index 0-135
// pai 0-33
// name (0-3, 0-8)

pub type Name = (u8, usize);

pub trait Yaku {
    fn is_yaku(&self, ba: usize, cha: usize) -> bool;
    fn is_yaochu(&self) -> bool;
    fn is_jun(&self) -> bool;
}

impl Yaku for Name {
    fn is_yaku(&self, ba: usize, cha: usize) -> bool {
        self.0 == 3 && (self.1 >= 4 || self.1 == ba || self.1 == cha)
    }
    fn is_yaochu(&self) -> bool {
        self.0 == 3 || self.1 == 0 || self.1 == 8
    }
    fn is_jun(&self) -> bool {
        self.0 < 3 && (self.1 == 0 || self.1 == 8)
    }
}

pub fn index2pai(index: u8) -> u8 {
    index / 4
}

pub fn pai2name(pai: u8) -> Name {
    (pai / 9, (pai % 9) as usize)
}

pub fn index2name(index: u8) -> Name {
    pai2name(index2pai(index))
}

pub fn name2pai(name: Name) -> u8 {
    name.0 * 9 + name.1 as u8
}