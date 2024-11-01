use serde_json::Value;
pub fn string_values_of<'a, 'b, 'c>(target: &'a Value, key1: &'b str, key2: &'c str) -> &'a str {
    target.get(key1).unwrap().get(key2).unwrap().as_str().unwrap()
}

pub fn string_value_of<'a, 'b>(target: &'a Value, key: &'b str) -> &'a str {
    target.get(key).unwrap().as_str().unwrap()
}