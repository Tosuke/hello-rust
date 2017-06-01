use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    println!("{}", JSON::JSONBoolean(true).serialize());
    println!("{}", JSON::JSONNumber(3.14926535).serialize());
    println!("{}", JSON::JSONString(String::from("hoge")).serialize());
    println!("{}", JSON::JSONArray(vec![
        JSON::JSONBoolean(false),
        JSON::JSONNumber(114514.0),
        JSON::JSONString(String::from("piyo"))
    ]).serialize());

    let mut object = HashMap::new();
    object.insert("number".to_string(), JSON::JSONNumber(5000000000000000.0));
    object.insert("boolean".to_string(), JSON::JSONBoolean(true));

    println!("{}", JSON::JSONObject(object).serialize());
}

enum JSON {
    JSONObject(HashMap<String, JSON>),
    JSONArray(Vec<JSON>),
    JSONBoolean(bool),
    JSONNumber(f64),
    JSONString(String)
}

impl JSON {
    fn serialize(&self) -> String {
        match *self {
            JSON::JSONObject(ref a) => {
                let mut a = a.iter().map(|(key, value)| format!("\"{}\":{}", key, value.serialize()));
                match a.next() {
                    None        => "{}".to_string(),
                    Some(a1)    => {
                        let result = a.fold(a1, |a, b| a + "," + &b);
                        format!("{{{}}}", result)
                    }
                }
            },
            JSON::JSONArray(ref a)  => {
                let mut a = a.into_iter().map(|a| a.serialize());
                match a.next() {
                    None        => "[]".to_string(),
                    Some(a1)    => {
                        let result = a.fold(a1, |a, b| a + "," + &b);
                        format!("[{}]", result)
                    } 
                }
            },
            JSON::JSONBoolean(a)    => format!("{}", a),
            JSON::JSONNumber(a)     => format!("{}", a),
            JSON::JSONString(ref a) => format!("\"{}\"", a),
        }
    }
}