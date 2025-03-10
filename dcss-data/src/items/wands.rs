#[derive(Clone, Debug)]
pub(crate) struct Wand {
    pub(crate) data_collected: bool,
}

impl Wand {
    pub(crate) fn new() -> Self {
        Self {
            data_collected: false,
        }
    }
}
