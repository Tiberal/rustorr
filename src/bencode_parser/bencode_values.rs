use indexmap::IndexMap;

#[derive(Clone, Debug)]
pub(crate) enum Value {
    String(String),
    Number(i64),
    Array(Vec<Value>),
    Dictionary(IndexMap<String, Value>),
}