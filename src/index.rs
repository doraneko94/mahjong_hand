// index 0-135
// pai 0-33
// name (0-3, 0-8)

pub type Name = (u8, usize);

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