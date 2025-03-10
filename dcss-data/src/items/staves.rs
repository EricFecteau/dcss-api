#[derive(Clone, Debug)]
pub(crate) struct Staff {
    pub(crate) data_collected: bool,
}

impl Staff {
    pub(crate) fn new() -> Self {
        Self {
            data_collected: false,
        }
    }
}
