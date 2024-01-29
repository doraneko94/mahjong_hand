use crate::index::Name;

#[derive(Clone)]
pub struct Naki {
    pub kan: Vec<Name>
}

impl Naki {
    pub fn new() -> Self {
        Self { kan: Vec::new() }
    }
}