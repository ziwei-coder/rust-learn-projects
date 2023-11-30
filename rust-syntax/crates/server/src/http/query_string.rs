use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: &'buf str, val: &'buf str) {
        self.data
            .entry(key)
            .and_modify(|existing| match existing {
                Value::Multiple(vec) => vec.push(val),
                Value::Single(pre_val) => {
                    let vec: Vec<&str> = vec![pre_val, val];
                    *existing = Value::Multiple(vec);
                }
            })
            .or_insert(Value::Single(val));
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    // a=1&b=2&c&d=&e===&d=7&d=abc
    fn from(s: &'buf str) -> Self {
        let mut query_string = QueryString {
            data: HashMap::new(),
        };

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut value = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i + 1..];
            }

            query_string.set(key, value);
        }

        query_string
    }
}
