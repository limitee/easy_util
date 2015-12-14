extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;

use std::str::FromStr;

#[macro_export]
macro_rules! json {
    ($e:expr) => (Json::from_str($e).unwrap());
}

#[macro_export]
macro_rules! json_path {
    ($o:expr;$($y:expr),*) => {{
        let mut data = $o;
        $(
            if data.is_array() {
                let array = data.as_array().unwrap();
                let index = usize::from_str($y).unwrap();
                data = array.get(index).unwrap();
            } else {
                data = data.find($y).unwrap();
            }
        )*
        data
    }}
}

#[macro_export]
macro_rules! json_str {
    ($o:expr;$($y:expr),*) => {{
        let data = json_path!($o;$($y),*);    
        data.as_string().unwrap()
    }}
}

#[macro_export]
macro_rules! json_set {
    ($o:expr;$k:expr;$v:expr) => {{
        let obj = $o.as_object_mut().unwrap();
        obj.insert($k.to_string(), $v.to_json());
    }}
}

#[macro_export]
macro_rules! json_i64 {
    ($o:expr;$($y:expr),*) => {{
        let data = json_path!($o;$($y),*);    
        if data.is_string() {
            let my_str = data.as_string().unwrap();
            i64::from_str(my_str).unwrap()
        } else {
            data.as_i64().unwrap()
        }
    }}
}
