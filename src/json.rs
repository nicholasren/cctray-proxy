use serde_json::Value;

pub fn has_value_by_one_key(target: &Value, key: &str, expected: &str) -> bool {
    let value = target.get(key)
        .and_then(|v| v.as_str());
    value.is_some() && value.unwrap() == expected
}

pub fn has_value_by_two_keys(target: &Value, key1: &str, key2: &str, expected: &str) -> bool {
    let value = try_find_value_by_two_keys(target, key1, key2);
    value.is_some() && value.unwrap() == expected
}

pub fn has_value_by_three_keys(target: &Value, key1: &str, key2: &str, key3: &str, expected: &str) -> bool {
    let value = target
        .get(key1)
        .and_then(|v| v.get(key2))
        .and_then(|v| v.get(key3))
        .and_then(|v| v.as_str());

    value.is_some() && value.unwrap() == expected
}

pub fn string_value_by_two_keys<'a, 'b, 'c>(target: &'a Value, key1: &'b str, key2: &'c str) -> &'a str {
    let value = try_find_value_by_two_keys(target, key1, key2);
    value.unwrap_or("unknown")
}

fn try_find_value_by_two_keys<'a, 'b, 'c>(target: &'a Value, key1: &'b str, key2: &'c str) -> Option<&'a str> {
    let value = target.get(key1)
        .and_then(|v| v.get(key2))
        .and_then(|v| v.as_str());
    value
}
