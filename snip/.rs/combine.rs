use combine::{
    Stream, choice, from_str, many1, parser,
    satisfy, sep_by, skip_many1, token,
};
use combine::parser::char::{digit, space};

#[derive(Debug, Default, PartialEq)]
struct Record {
    id: Option<i32>,
    info: String,
}

// define how to update itself each time an item appears
impl std::iter::Extend<Self> for Record {
    fn extend<T: IntoIterator<Item=Self>>(&mut self, iter: T) {
        for item in iter {
            if let Some(i) = item.id {
                self.id = Some(i)
            }
            if !item.info.is_empty() {
                if !self.info.is_empty() {
                    self.info.push(' ');
                }
                self.info.push_str(&item.info)
            }
        }
    }
}
parser! {
    fn record_[Input]()(Input) -> Record
    where [ Input: Stream<Token = char> ] {
        sep_by(record_unit_(), spaces1_())
    }
}
parser! {
    fn record_unit_[Input]()(Input) -> Record
    where [ Input: Stream<Token = char> ] {
        choice((
            token('#').with(non_nega_i_()).map(|i| {
                let mut record = Record::default();
                record.id = Some(i);
                record
            }),
            graphics1_().map(|g| {
                let mut record = Record::default();
                record.info = g;
                record
            }),
        ))
    }
}
parser! {
    fn spaces1_[Input]()(Input) -> ()
    where [ Input: Stream<Token = char> ] {
        skip_many1(space())
    }
}
parser! {
    fn non_nega_i_[Input]()(Input) -> i32
    where [ Input: Stream<Token = char> ] {
        from_str(many1::<String, _, _>(digit()))
    }
}
parser! {
    fn graphics1_[Input]()(Input) -> String
    where [ Input: Stream<Token = char> ] {
        many1(satisfy(|c: char| !c.is_whitespace() && !c.is_control()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn t_record_() {
        let trial = record_().easy_parse(
            "cool   #3 #55 \r\n simple   \n   #777 beautiful"
        );
        assert_eq!(trial, Ok((
            Record {
                id: Some(777),
                info: String::from("cool simple beautiful"),
            },
            ""))
        );
    }
}
