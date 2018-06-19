use nom::types::CompleteStr;
use nom::Needed;
use json::Value;
use std::collections::HashMap;
use std::iter::FromIterator;

named!(string<CompleteStr, String>,
        delimited!(
            tag!("\""),
            escaped_transform!(
                none_of!("\\\""),
                '\\',
                alt!(
                    tag!("\\") => { |_| "\\" }
                  | tag!("\"") => { |_| "\"" }
                )),
            tag!("\""))
);

named!(literal_true<CompleteStr, Value>,
    value!(Value::True, tag!("true"))
);

named!(literal_false<CompleteStr, Value>,
    value!(Value::False, tag!("false"))
);

named!(literal_null<CompleteStr, Value>,
    value!(Value::Null, tag!("null"))
);

named!(number<CompleteStr, CompleteStr>,
    re_find!(r"(0|-?[1-9]+)(\.\d+)?((e|E)(\+|-)?\d*)?")
);

named!(json_value<CompleteStr, Value>,
    alt!(
        string => { Value::String }
        | object => { Value::Object }
        | array
        | literal_true
        | literal_false
        | literal_null
        | number => { |n: CompleteStr| Value::Number(n.to_string()) }
    )
);

named!(pair<CompleteStr, (String, Value)>,
    separated_pair!(
        string,
        ws!(char!(':')),
        json_value
    )
);

named!(array<CompleteStr, Value>,
    map!(
        delimited!(
            ws!(char!('[')),
            separated_list!(
                ws!(char!(',')),
                json_value
            ),
            ws!(char!(']'))
        ),
        |vec| { Value::Array(vec) }
    )
);

named!(pub object<CompleteStr, HashMap<String, Value>>,
    map!(
        delimited!(
            ws!(char!('{')),
            separated_list_complete!(
                ws!(char!(',')),
                pair
            ),
            ws!(char!('}'))
        ),
        |vec: Vec<(String, Value)>| { HashMap::from_iter(vec.into_iter()) }
    )
);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_string() {
        let string_test = string(CompleteStr("\"Hallo Welt!\""));
        assert_eq!(string_test, Ok((CompleteStr(""), String::from("Hallo Welt!"))))
    }

    #[test]
    fn parse_string_pair() {
        let pair_string = "\"manufacturer\": \"BMW\"";
        let pair_test = pair(CompleteStr(pair_string));
        assert_eq!(pair_test, Ok((CompleteStr(""), (String::from("manufacturer"),
                                                    Value::String(String::from("BMW"))))))
    }

    #[test]
    fn parse_literal_true() {
        let literal_test = literal_true(CompleteStr("true"));
        assert_eq!(literal_test, Ok((CompleteStr(""), Value::True)))
    }

    #[test]
    fn parse_value() {
        let value_test = json_value(CompleteStr("null"));
        assert_eq!(value_test, Ok((CompleteStr(""), Value::Null)))
    }

    #[test]
    fn parse_number() {
        assert_eq!(number(CompleteStr("-42.24e+10")), Ok((CompleteStr(""), CompleteStr("-42.24e+10"))));
    }

    #[test]
    fn parse_array() {
        let array_test = array(CompleteStr("[\"BMW\", \"Jaguar\"]"));
        assert_eq!(array_test, Ok((CompleteStr(""), Value::Array(vec![Value::String(String::from("BMW")),
                                                                      Value::String(String::from("Jaguar"))]))))
    }

    #[test]
    fn parse_object() {
        let object_string =
            "{ \"manufacturer\": \"BMW\", \"model\": \"1 Series\", \"hatchback\": true, \"hp\": 143 }";
        let object_test = object(CompleteStr(object_string));

        let vec = vec![
            ("manufacturer".to_string(), Value::String("BMW".to_string())),
            ("model".to_string(), Value::String("1 Series".to_string())),
            ("hatchback".to_string(), Value::True),
            ("hp".to_string(), Value::Number("143".to_string()))
        ];

        let map: HashMap<String, Value> = HashMap::from_iter(vec.into_iter());
        assert_eq!(object_test, Ok((CompleteStr(""), map)))
    }
}
