pub struct SearchKey {
    pub search_string: String,
    pub len: String,
}

impl SearchKey {
    pub fn search_string(&self) -> &str {
        &self.search_string
    }

    pub fn search_len(&self) -> &str {
        &self.len
    }
}
