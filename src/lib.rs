mod index;
pub use {
    index::{Index, IntoIndex},
    serde_json::{Value, from_str, from_slice, from_reader, Error as SerdeJsonError}
};

pub fn get_value_dyn<'v, 'i>(value: &'v serde_json::Value, idxs: &'i [&dyn IntoIndex]) -> Option<&'v serde_json::Value> {

    let idxs = idxs.iter().map(|i| i.to_index()).collect::<Vec<Index>>();
    get_value(value, idxs.as_slice())

}

pub fn get_value<'v, 'i>(value: &'v serde_json::Value, idxs: &'i [Index]) -> Option<&'v serde_json::Value> {

    match idxs[0] {
        Index::Key(ref k) => match value.get(k) {
            Some(val) => if idxs.len() > 1 {
                get_value(val, &idxs[1..])
            } else {
                Some(val)
            },
            None => None
        },
        Index::Position(i) => match value.get(i) {
            Some(val) => if idxs.len() > 1 {
                get_value(val, &idxs[1..])
            } else {
                Some(val)
            },
            None => None
        }
    }

}
