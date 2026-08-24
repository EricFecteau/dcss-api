#[derive(Clone, Debug)]
pub(crate) struct Missile {
    pub(crate) data_collected: bool,
    pub(crate) letter: char,
}

impl Missile {
    pub(crate) fn new(letter: char) -> Self {
        Self {
            data_collected: false,
            letter,
        }
    }
}
