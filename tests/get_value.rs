use djson;

#[test]
fn get_value_1() {

    let json_str = include_str!("test.json");
    let value: djson::Value = djson::from_str(json_str).unwrap();
    let idxs = djson::Index::parse("one").unwrap();

    let res = djson::get_value(&value, &idxs);

    assert_eq!(res.map(|v| v.as_i64()), Some(Some(1)));

}

#[test]
fn get_value_2() {

    let json_str = include_str!("test.json");
    let value: djson::Value = djson::from_str(json_str).unwrap();
    let idxs = djson::Index::parse("three").unwrap();

    let res = djson::get_value(&value, &idxs);

    assert_eq!(res.map(|v| v.is_object()), Some(true));

}
