use combine::*;
use combine::parser::{
    char::{digit, space, spaces},
    choice::Optional,
    combinator::Try,
    sequence::With,
};

#[derive(Debug, Default, Eq, PartialEq)]
struct Condition {
    id: Option<i32>,
    words: Vec<String>,
}

parser! {
    fn a_condition[Input]()(Input) -> Condition
    where [ Input: Stream<Token = char> ] {
        let opt: fn() -> Optional<Try<With<_, With<_, self::a_condition<Input>>>>>
        = || optional(attempt(
            space().with(spaces().with(a_condition()))
        ));
        choice((
            token('#').with(a_decimal()).and(opt()).map(|(id, opt)| {
                let mut condition = opt.unwrap_or_default();
                condition.id = Some(id);
                condition
            }),
            a_word().and(opt()).map(|(w, opt)| {
                let mut condition = opt.unwrap_or_default();
                condition.words.push(w);
                condition
            }),
        ))
    }
}
parser! {
    fn a_decimal[Input]()(Input) -> i32
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_a_condition_ok() {
        let trial = a_condition().easy_parse(
            "cool\n#3 #55  \r\n simple\r\n\r\n\n\n\r  #777 beautiful   \r     "
        );
        let expect = Ok((
            Condition {
                id: Some(3),
                words: vec!["beautiful", "simple", "cool"].iter().map(|s| s.to_string()).collect(),
            },
            "   \r     "
        ));
        assert_eq!(trial, expect);
    }
    #[test]
    fn t_a_condition_err_10() {
        let trial = a_condition().easy_parse(
            " cool"
        );
        assert!(trial.is_err());
    }
    #[test]
    fn t_a_condition_err_20() {
        let trial = a_condition().easy_parse(
            "#\n777"
        );
        assert!(trial.is_err());
    }
}
