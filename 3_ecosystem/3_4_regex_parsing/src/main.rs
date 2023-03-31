use std::str::FromStr;

use const_format::concatcp;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use regex::Regex;

fn main() {
}

#[derive(Parser)]
#[grammar_inline = r##"
    identifier = { (ASCII_ALPHA | "_") ~ ASCII_ALPHANUMERIC* }
    integer = { ASCII_DIGIT+ }
    argument = ${ identifier | integer }
    parameter = ${ argument ~ "$" }
    count = ${ parameter | integer }

    fill = { !align ~ ANY }
    align = { "<" | "^" | ">" }
    fill_align = ${ fill? ~ align }
    sign = { "-" | "+" }
    width = ${ count }
    precision = ${ count | "*" }
    dot_precision = ${ "." ~ precision }
    type = ${  "?" | "x?" | "X?" | identifier }

    format_spec = ${ fill_align? ~ sign? ~ "#"? ~ "0"? ~ width? ~ dot_precision? ~ type? }
"##]
struct FmtParser;

const FMT_REGEX_SPEC: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    const IDENTIFIER: &str = r"(?:\p{XID_Start}\p{XID_Continue}*)|(?:_\p{XID_Continue})";
    const INTEGER: &str = r"(?:\d+)";
    const ARGUMENT: &str = concatcp!(IDENTIFIER, r"|", INTEGER);
    const PARAMETER: &str = concatcp!(ARGUMENT, r"\$");
    const COUNT: &str = concatcp!(r"(?:", PARAMETER, r")|(?:", INTEGER, r")");
    
    const FILL_ALIGN: &str = r"(.?(?:[<\^>]))?";
    const SIGN: &str = r"(-|\+)?";
    const WIDTH: &str = concatcp!(r"(", COUNT, r")?");
    const PRECISION: &str = concatcp!(r"(", COUNT, r"|\*", r")?");
    const TYPE: &str = concatcp!(r"(\?|(?:[xX]\?)|", IDENTIFIER, r")?");

    const FORMAT_SPEC: &str = concatcp!(r"^", FILL_ALIGN, SIGN, r"(#)?(0)?", WIDTH, r"(?:\.", PRECISION, r")", TYPE, r"$");

    Regex::new(FORMAT_SPEC).unwrap()
});

fn parse_regex(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    let res = match FMT_REGEX_SPEC.captures(input) {
        Some(res) => res,
        None => return (None, None, None),
    };

    (
        res.get(2).and_then(|sign| sign.as_str().parse().ok()),
        res.get(5).and_then(|width| width.as_str().parse().ok()),
        res.get(6).and_then(|precision| precision.as_str().parse().ok()),
    )
}

fn parse_parser(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    let res = match FmtParser::parse(Rule::format_spec, input) {
        Ok(mut res) => res.next().unwrap(),
        Err(_) => return (None, None, None),
    };
    println!("res={:?}", res.clone().into_inner().into_iter().collect::<Vec<_>>());

    fn parse_dot_precision(entry: Pair<Rule>) -> Option<Precision> {
        for dot_prec_item in entry.into_inner() {
            if let Rule::precision = dot_prec_item.as_rule() {
                return dot_prec_item.as_str().parse().ok()
            }
        }

        None
    }
    
    fn parse_format_spec(entry: Pair<Rule>) -> (Option<Sign>, Option<usize>, Option<Precision>) {
        let mut sign = None;
        let mut width = None;
        let mut presicion = None;

        for item in entry.into_inner() {
            match item.as_rule() {
                Rule::sign => sign = item.as_str().parse().ok(),
                Rule::width => width = item.as_str().parse().ok(),
                Rule::dot_precision => presicion = parse_dot_precision(item),
                _ => continue
            }
        }

        (sign, width, presicion)
    }

    return parse_format_spec(res)
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

impl FromStr for Sign {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Minus),
            "+" => Ok(Self::Plus),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

impl FromStr for Precision {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            println!("yo");
            return Ok(Self::Asterisk)
        }

        if s.ends_with("$") {
            return Ok(Self::Argument(s[..s.len()-1].trim_start_matches('0').parse().map_err(|_| ())?))
        }

        return Ok(Self::Integer(s.trim_start_matches('0').parse().map_err(|_| ())?))
    }
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse_regex(input);
            assert_eq!(sign, expected);

            let (sign, ..) = parse_parser(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse_regex(input);
            assert_eq!(width, expected);

            let (_, width, _) = parse_parser(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse_regex(input);
            assert_eq!(precision, expected);

            let (_, _, precision) = parse_parser(input);
            assert_eq!(precision, expected);
        }
    }
}
