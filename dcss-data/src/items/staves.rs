#[derive(Clone, Debug)]
pub(crate) struct Staff {
    pub(crate) data_collected: bool,
    pub(crate) letter: char,
}

impl Staff {
    pub(crate) fn new(letter: char) -> Self {
        Self {
            data_collected: false,
            letter,
        }
    }
}
