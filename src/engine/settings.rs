pub struct Settings {
    pub tt_size: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self { tt_size: 16 }
    }
}
