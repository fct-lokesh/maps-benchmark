pub struct LinearMapV3 {
    map: Vec<(String, String)>,
}

impl LinearMapV3 {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }

    pub fn find(&self, key: &str) -> Option<&str> {
        self.map
            .iter()
            .find(|(k, _)| k == key)
            .map(move |(_, val)| val.as_str())
    }

    pub fn push(&mut self, key: String, val: String) {
        self.map.push((key, val));
    }
}
