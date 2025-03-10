#[derive(Clone, Debug)]
pub(crate) struct Missile {
    pub(crate) data_collected: bool,
}

impl Missile {
    pub(crate) fn new() -> Self {
        Self {
            data_collected: false,
        }
    }
}
