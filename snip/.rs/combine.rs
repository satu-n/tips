use combine::{
    Stream, choice, eof, from_str,
    many1, parser, satisfy, skip_many1, token,
};
use combine::parser::{
    char::{digit, space},
};

#[derive(Debug, Default, Eq, PartialEq)]
struct Condition {
    id: Option<i32>,
    words: Vec<String>,
}

parser! {
    fn a_condition[Input]()(Input) -> Condition
    where [ Input: Stream<Token = char> ] {
        let opt = || choice((
            eof().map(|_| Condition::default()),
            a_spaces1().with(a_condition()),
        ));
        choice((
            opt(),
            token('#').with(a_non_nega_i()).and(opt()).map(|(i, mut opt)| {
                opt.id = Some(i);
                opt
            }),
            a_word().and(opt()).map(|(w, mut opt)| {
                opt.words.push(w);
                opt
            }),
        ))
    }
}
parser! {
    fn a_non_nega_i[Input]()(Input) -> i32
    where [ Input: Stream<Token = char> ] {
        from_str(many1::<String, _, _>(digit()))
    }
}
parser! {
    fn a_word[Input]()(Input) -> String
    where [ Input: Stream<Token = char> ] {
        many1(satisfy(|c: char| !c.is_whitespace() && !c.is_control()))
    }
}
parser! {
    fn a_spaces1[Input]()(Input) -> ()
    where [ Input: Stream<Token = char> ] {
        skip_many1(space())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn t_a_condition() {
        let trial = a_condition().easy_parse(
            "\n   cool   #3 #55 \r\n simple   \n   #777 beautiful   \n   "
        );
        assert_eq!(trial, Ok((
            Condition {
                id: Some(3),
                words: vec![
                    String::from("beautiful"),
                    String::from("simple"),
                    String::from("cool"),
                    ],
                },
                ""
            ))
        );
    }
}
