use indexmap::IndexMap;
use crate::bencode_parser::bencode_values::Value;

pub(crate) trait Collector {
    fn insert(&mut self, value: Value);
    fn result(&self) -> CollectorResult;
}

pub enum CollectorResult {
    Array(Vec<Value>),
    Dictionary(IndexMap<String, Value>),
}


pub(crate) struct ArrayCollector {
    array: Vec<Value>,
}

impl ArrayCollector {
    pub(crate) fn new() -> Self {
        Self {
            array: vec![],
        }
    }
}

impl Collector for ArrayCollector {
    fn insert(&mut self, value: Value) {
        self.array.push(value);
    }

    fn result(&self) -> CollectorResult {
        CollectorResult::Array(self.array.clone())
    }
}

pub(crate) struct DictionaryCollector {
    map: IndexMap<String, Value>,
    current_key: Option<String>,
}

impl DictionaryCollector {
    pub(crate) fn new() -> Self {
        Self {
            map: IndexMap::new(),
            current_key: None,
        }
    }
}

impl Collector for DictionaryCollector {
    fn insert(&mut self, value: Value) {
        let is_insert_ready = self.current_key.is_some();
        if is_insert_ready {
            let key = self.current_key.clone().unwrap();
            self.map.insert(key, value);
            self.current_key = None;
        } else {
            if let Value::String(string) = value {
                self.current_key = Some(string);
            } else {
                panic!("Only string is eligible as key");
            }
        }
    }

    fn result(&self) -> CollectorResult {
        CollectorResult::Dictionary(self.map.clone())
    }
}