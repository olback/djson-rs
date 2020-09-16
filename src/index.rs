pub trait IntoIndex {
    fn to_index(&self) -> Index;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Index {
    Key(String),
    Position(usize)
}

impl Index {

    pub fn new(idx: &dyn IntoIndex) -> Self {
        idx.to_index()
    }

    pub fn parse(idx_str: &str) -> Result<Vec<Index>, &'static str> {

        let mut idx_parts = Vec::<Index>::new();
        let split_parts = idx_str.split('.').collect::<Vec<&str>>();

        for part in split_parts {
            if part.contains('[') && part.contains(']') {
                let a = part.find('[').unwrap();
                let b = part.find(']').unwrap();
                if !&part[0..a].is_empty() {
                    idx_parts.push(Index::new(&&part[0..a]));
                }
                let usize_idx = part[a+1..b].parse::<usize>().map_err(|_| "Error parsing index as usize")?;
                idx_parts.push(Index::new(&usize_idx));
            } else if part.contains('[') || part.contains(']') {
                return Err("Invalid syntax")
            } else {
                idx_parts.push(Index::new(&part));
            }
        }

        Ok(idx_parts)

    }

}

impl IntoIndex for &str {
    fn to_index(&self) -> Index {
        Index::Key(String::from(*self))
    }
}

impl IntoIndex for String {
    fn to_index(&self) -> Index {
        Index::Key(self.clone())
    }
}

impl IntoIndex for usize {
    fn to_index(&self) -> Index {
        Index::Position(*self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn impl_into_index_usize() {
        assert_eq!(0usize.to_index(), Index::Position(0));
        assert_eq!(Index::new(&0usize), Index::Position(0));
    }

    #[test]
    fn impl_into_index_str() {
        assert_eq!("key".to_index(), Index::Key(String::from("key")));
        assert_eq!(Index::new(&"key"), Index::Key(String::from("key")));
    }

    #[test]
    fn impl_into_index_string() {
        assert_eq!(String::from("key").to_index(), Index::Key(String::from("key")));
        assert_eq!(Index::new(&String::from("key")), Index::Key(String::from("key")));
    }

    #[test]
    fn parse_1() {
        let i1 = Index::parse("one");
        let i2 = vec![Index::Key("one".into())];
        assert_eq!(i1, Ok(i2));
    }

    #[test]
    fn parse_2() {
        let i1 = Index::parse("one.two");
        let i2 = vec![Index::Key("one".into()), Index::Key("two".into())];
        assert_eq!(i1, Ok(i2));
    }

    #[test]
    fn parse_3() {
        let i1 = Index::parse("one.two[0]");
        let i2 = vec![Index::Key("one".into()), Index::Key("two".into()), Index::Position(0)];
        assert_eq!(i1, Ok(i2));
    }

    #[test]
    fn parse_4() {
        let i1 = Index::parse("one.two[0].one");
        let i2 = vec![Index::Key("one".into()), Index::Key("two".into()), Index::Position(0), Index::Key("one".into())];
        assert_eq!(i1, Ok(i2));
    }

    // #[test]
    // fn parse_5() {
    //     let i1 = Index::parse("one.two[0][0]");
    //     let i2 = vec![Index::Key("one".into()), Index::Key("two".into()), Index::Position(0), Index::Position(0)];
    //     assert_eq!(i1, Ok(i2));
    // }

    #[test]
    fn parse_6() {
        let i1 = Index::parse("[0]");
        let i2 = vec![Index::Position(0)];
        assert_eq!(i1, Ok(i2));
    }

}
