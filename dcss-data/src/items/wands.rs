#[derive(Clone, Debug)]
pub(crate) struct Wand {
    pub(crate) data_collected: bool,
    pub(crate) letter: char,
}

impl Wand {
    pub(crate) fn new(letter: char) -> Self {
        Self {
            data_collected: false,
            letter,
        }
    }
}
