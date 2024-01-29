pub struct Score {
    pub parent: Option<usize>,
    pub children: usize,
    pub fu: usize,
    pub han: usize,
}

fn _marume(x: usize, u: usize) -> usize {
    (x + u - 1) / u * u
}

impl Score {
    pub fn new(fu: usize, han: usize, oya: bool) -> Self {
        let f = _marume(fu, 10);
        let core = if han >= 13 { 8000 }
            else if han >= 11 { 6000 }
            else if han >= 8 { 4000 }
            else if han >= 6 { 3000 }
            else {
                let c = f * 2_usize.pow(han as u32 + 2);
                if c >= 2000 { 2000 } else { c }
        };
        
        let (parent, children) = if oya {
            (None, _marume(core * 2, 100))
        } else {
            (Some(_marume(core * 2, 100)), _marume(core, 100))
        };
        Self { parent, children, han, fu }
    }
}