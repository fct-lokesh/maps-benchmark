use std::collections::HashMap;

pub struct JSM {
    datas: HashMap<String, String>,
}

impl JSM {
    pub fn new() -> Self {
        Self {
            datas: HashMap::new(),
        }
    }

    pub fn find(&self, data: &str) -> Option<&str> {
        self.datas.get(data).map(|s| s.as_str())
    }
    pub fn push(&mut self, key: String, val: String) {
        self.datas.insert(key, val);
    }
}
