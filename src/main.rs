#[macro_use]
extern crate easy_util;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

use std::str::FromStr;

fn main() {
    let my_str = format!(r#"
        {{
            "a": [
            {{
                "index":0,
                "content":"nothing"
            }},
            {{
                "index":1,
                "content":"something"
            }},
            {{
                "index":"2"
            }}
            ]
        }}
    "#);
    let my_json = json!(&my_str);
    let after_json = json_i64!(&my_json; "a", "2", "index");
    println!("{}", after_json);
    let content = json_str!(&my_json; "a", "1", "content");
    println!("{}", content);
}
