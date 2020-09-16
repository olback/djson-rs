# djson - Access Dynamic JSON in Rust

Example:
```rust
let json_str = include_str!("../test.json");

let value: Value = djson::from_str(json_str).unwrap();
let idxs = Index::parse("four[2].one").unwrap();

let res = djson::get_value(&value, &idxs);
println!("{:#?}", res);

let res = djson::get_value_dyn(&value, &[&"four", &2, &"one"]);
println!("{:#?}", res);
```
